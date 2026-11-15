"""Host readiness boundaries, with a real local TLS server for pin checks."""
import contextlib
import hashlib
import http.server
import json
import os
from pathlib import Path
import ssl
import subprocess
import sys
import tempfile
import threading
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch
import installer


class ReadinessTests(unittest.TestCase):
    def setUp(self):
        control=patch.object(installer,'HostControl');self.addCleanup(control.stop)
        control.start().return_value.status.return_value={'active':None}
    @classmethod
    def setUpClass(cls):
        cls.directory=tempfile.TemporaryDirectory();base=Path(cls.directory.name)
        cls.certificate=base/'unit.pem';cls.key=base/'unit.key'
        subprocess.run(['openssl','req','-x509','-newkey','rsa:2048','-nodes','-days','1',
            '-subj','/CN=local readiness unit fixture','-keyout',str(cls.key),'-out',str(cls.certificate)],
            check=True,capture_output=True)
        cls.fingerprint=hashlib.sha256(ssl.PEM_cert_to_DER_cert(cls.certificate.read_text())).hexdigest()

    @classmethod
    def tearDownClass(cls):
        cls.directory.cleanup()

    @contextlib.contextmanager
    def server(self,body=b'{"status":"setup-required"}',code=200):
        requests=[]
        class Handler(http.server.BaseHTTPRequestHandler):
            def do_GET(self):
                requests.append(self.path)
                self.send_response(code);self.send_header('Content-Length',str(len(body)));self.end_headers()
                self.wfile.write(body)
            def log_message(self,*args):
                pass
        server=http.server.ThreadingHTTPServer(('127.0.0.1',0),Handler)
        context=ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER);context.load_cert_chain(self.certificate,self.key)
        server.socket=context.wrap_socket(server.socket,server_side=True)
        thread=threading.Thread(target=server.serve_forever,daemon=True);thread.start()
        port=server.server_address[1]
        try:
            yield {'schemaVersion':1,'httpAddress':'127.0.0.1:'+str(port),'httpPort':port,
                   'certificateFingerprint':self.fingerprint},requests
        finally:
            server.shutdown();server.server_close();thread.join(timeout=2)
            self.assertFalse(thread.is_alive())

    def test_real_tls_health_requires_the_exact_private_host_leaf_pin(self):
        with self.server() as (description,requests):
            self.assertTrue(installer.probe_listener(description))
            self.assertEqual(requests,['/health/live'])

    def test_wrong_peer_pin_is_rejected_before_sending_any_http_request(self):
        with self.server() as (description,requests):
            description['certificateFingerprint']='0'*64
            self.assertFalse(installer.probe_listener(description))
            self.assertEqual(requests,[])

    def test_matching_tls_without_valid_live_response_is_not_ready(self):
        for body,code in [(b'{"status":"setup-required"}',503),(b'[]',200),(b'x'*1025,200)]:
            with self.subTest(code=code,bodyLength=len(body)),self.server(body,code) as (description,requests):
                self.assertFalse(installer.probe_listener(description))

    def service(self,state='active'):
        service=Mock();service.unit='test-only-center.service'
        service.systemctl.return_value=SimpleNamespace(stdout='ActiveState='+state+'\nMainPID=123\nJob=\n')
        service.overrides.return_value={}
        return service

    def test_old_executable_without_declared_status_capability_is_never_probed(self):
        service=self.service();service.verify_local_command.side_effect=RuntimeError('missing capability')
        result=installer.status(service)
        self.assertFalse(result['ready']);self.assertEqual(result['readiness'],'UNVERIFIED')
        service.bound_command.assert_not_called();service.activate.assert_not_called()

    def test_nonactive_service_status_does_not_execute_or_start_the_application(self):
        service=self.service('activating')
        result=installer.status(service)
        self.assertFalse(result['ready'])
        service.verify_local_command.assert_not_called();service.bound_command.assert_not_called()
        service.activate.assert_not_called()

    def test_stale_active_executable_cannot_certify_installed_code_readiness(self):
        with tempfile.TemporaryDirectory() as directory:
            process=Path(directory)/'unit-only-old-executable';process.write_bytes(b'old unit fixture')
            service=self.service();service.path.return_value=process
            service.verify_local_command.return_value={'binarySha256':'0'*64}
            with patch.object(installer,'active_process_digest',return_value=hashlib.sha256(process.read_bytes()).hexdigest()):
                result=installer.status(service)
            self.assertFalse(result['ready']);self.assertEqual(result['readiness'],'UNVERIFIED')
            service.bound_command.assert_not_called();service.activate.assert_not_called()

    def test_current_process_and_pinned_live_probe_select_saved_port_without_a_start(self):
        with tempfile.TemporaryDirectory() as directory:
            process=Path(directory)/'unit-only-current-executable';process.write_bytes(b'current unit fixture')
            service=self.service();service.path.return_value=process
            service.verify_local_command.return_value={'binarySha256':hashlib.sha256(process.read_bytes()).hexdigest()}
            description={'schemaVersion':1,'httpAddress':'[::]:28443','httpPort':28443,'certificateFingerprint':'a'*64}
            service.bound_command.return_value=description
            with patch.object(installer,'active_process_digest',return_value=hashlib.sha256(process.read_bytes()).hexdigest()),patch.object(installer,'probe_listener',return_value=True) as probe:
                result=installer.status(service)
            self.assertTrue(result['ready']);self.assertEqual(result['httpPort'],28443)
            service.bound_command.assert_called_once_with('installation-status',include_overrides=True)
            probe.assert_called_once_with(description);service.activate.assert_not_called()

    @unittest.skipUnless(Path('/proc/self/exe').exists(),'Linux kernel process magic link')
    def test_actual_kernel_process_identity_is_read_without_relaxing_owned_path_guard(self):
        self.assertEqual(installer.active_process_digest(str(os.getpid())),hashlib.sha256(Path(sys.executable).read_bytes()).hexdigest())
        with self.assertRaisesRegex(RuntimeError,'Invalid'):
            installer.active_process_digest('../self')


if __name__=='__main__':
    unittest.main()

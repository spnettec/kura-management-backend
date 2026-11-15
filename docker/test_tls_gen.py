"""Exercise the generated PKI with the local default OpenSSL and TLS policy."""
import hashlib
from pathlib import Path
import shutil
import socket
import ssl
import subprocess
import tempfile
import threading
import unittest


class GeneratedPkiTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory(prefix='yofc-reviewed-pki-')
        self.root = Path(self.directory.name)
        shutil.copyfile(Path(__file__).with_name('tls-gen.sh'), self.root / 'tls-gen.sh')
        self.generate()

    def tearDown(self):
        self.directory.cleanup()

    def generate(self, check=True):
        return subprocess.run(['/bin/sh', str(self.root / 'tls-gen.sh'), 'review-client'],
                              capture_output=True, text=True, check=check)

    def test_sha256_certificates_and_repeat_retains_every_file(self):
        for name in ['ca', 'server', 'review-client']:
            result = subprocess.check_output(['openssl', 'x509', '-in',
                str(self.root / 'tls' / (name + '.pem')), '-text', '-noout'], text=True)
            self.assertIn('Signature Algorithm: ecdsa-with-SHA256', result)
        before = {p.name: hashlib.sha256(p.read_bytes()).hexdigest()
                  for p in (self.root / 'tls').iterdir() if p.is_file()}
        self.assertNotEqual(self.generate(check=False).returncode, 0)
        self.assertEqual(before, {p.name: hashlib.sha256(p.read_bytes()).hexdigest()
                  for p in (self.root / 'tls').iterdir() if p.is_file()})
        for name in ['ca.key', 'server.key', 'review-client.key']:
            self.assertEqual((self.root / 'tls' / name).stat().st_mode & 0o777, 0o600)

    def test_default_security_policy_mutual_tls(self):
        tls = self.root / 'tls'
        server = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
        server.load_cert_chain(tls / 'server.pem', tls / 'server.key')
        server.load_verify_locations(tls / 'ca.pem')
        server.verify_mode = ssl.CERT_REQUIRED
        server.verify_flags |= ssl.VERIFY_X509_STRICT
        client = ssl.create_default_context(cafile=str(tls / 'ca.pem'))
        client.verify_flags |= ssl.VERIFY_X509_STRICT
        client.load_cert_chain(tls / 'review-client.pem', tls / 'review-client.key')
        errors = []
        with socket.socket() as listener:
            listener.bind(('127.0.0.1', 0))
            listener.listen(1)
            listener.settimeout(5)
            def serve():
                try:
                    peer, _ = listener.accept()
                    with peer, server.wrap_socket(peer, server_side=True) as session:
                        self.assertEqual(session.recv(2), b'ok')
                        session.sendall(b'ok')
                except Exception as error:
                    errors.append(error)
            worker = threading.Thread(target=serve)
            worker.start()
            try:
                with socket.create_connection(listener.getsockname(), timeout=5) as peer:
                    with client.wrap_socket(peer, server_hostname='localhost') as session:
                        session.sendall(b'ok')
                        self.assertEqual(session.recv(2), b'ok')
            finally:
                worker.join(6)
            self.assertFalse(worker.is_alive())
            self.assertEqual(errors, [])


if __name__ == '__main__':
    unittest.main()

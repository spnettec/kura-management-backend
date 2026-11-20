"""Host bootstrap receipts, public-certificate boundaries and local replay fences."""
import errno
import hashlib
import json
import os
from pathlib import Path
import socket
import ssl
import subprocess
import sys
import tempfile
import threading
import unittest
import uuid
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))
import host_ui
from setup import Setup

FIXTURES = Path(__file__).resolve().parents[1] / 'crates/console/tests/fixtures/tls-live'


class Service:
    def __init__(self):
        self.active = False
        self.starts = []
        self.failure = None
        self.job = ''

    def __call__(self, command, **kwargs):
        if command == host_ui.START:
            self.starts.append(list(command))
            self.active = True
            if isinstance(self.failure, Exception):
                raise self.failure
            return subprocess.CompletedProcess(command, self.failure or 0, '', '')
        if command[:3] == ['/usr/bin/systemctl', 'show', 'cockpit.socket']:
            return subprocess.CompletedProcess(command, 0,
                'LoadState=loaded\nActiveState=' + ('active' if self.active else 'inactive')
                + '\nSubState=' + ('listening' if self.active else 'dead') + '\nJob=' + self.job + '\n', '')
        raise AssertionError('Unexpected command')


@unittest.skipUnless(os.geteuid() == 0, 'root-owned host records')
class HostUiTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        (self.root / 'var/lib').mkdir(parents=True)
        self.service = Service()
        self.pem = (FIXTURES / 'server-one.pem').read_text()
        self.der = ssl.PEM_cert_to_DER_cert(self.pem)
        self.control = host_ui.HostUi(Setup(self.root, run=self.service), peer=lambda: self.der)
        self.certdir = self.root / host_ui.CERTIFICATES.lstrip('/')
        self.certdir.mkdir(parents=True)
        self.certificate = self.certdir / '0-self-signed.cert'
        self.certificate.write_text(self.pem)
        self.certificate.chmod(0o644)

    def identity(self):
        return str(uuid.uuid4())

    def test_inspection_of_an_inactive_host_creates_no_state_and_never_starts(self):
        value = self.control.snapshot()
        self.assertEqual(value['service']['ActiveState'], 'inactive')
        self.assertIsNone(value['certificate'])
        self.assertFalse(self.control.store.directory.exists())
        self.assertEqual(self.service.starts, [])

    def test_one_successful_start_has_an_original_receipt_and_is_never_resent(self):
        identity = self.identity()
        self.assertEqual(self.control.start(identity)['phase'], 'APPLIED')
        operation = self.control.operations()[0]
        self.assertEqual(operation['receipt']['command'], host_ui.START)
        self.assertEqual(operation['receipt']['operationId'], identity)
        self.assertFalse(self.control.start(identity)['startIssued'])
        self.assertFalse(self.control.start(self.identity())['startIssued'])
        self.assertEqual(self.service.starts, [host_ui.START])
        self.assertEqual(len(self.control.operations()), 1)
        self.assertEqual(self.control.snapshot()['certificate']['sha256'], hashlib.sha256(self.der).hexdigest())

    def test_live_socket_without_a_receipt_remains_unknown_and_blocks_new_ids(self):
        self.service.failure = subprocess.TimeoutExpired(host_ui.START, 30)
        identity = self.identity()
        self.assertEqual(self.control.start(identity)['phase'], 'UNKNOWN')
        file = self.control.store.directory / 'operations' / (identity + '.json')
        before = file.read_bytes()
        self.assertEqual(self.control.snapshot()['service']['ActiveState'], 'active')
        self.assertEqual(self.control.operations()[0]['phase'], 'UNKNOWN')
        self.assertFalse(self.control.start(identity)['startIssued'])
        self.assertFalse(self.control.start(self.identity())['startIssued'])
        self.assertEqual(file.read_bytes(), before)
        self.assertEqual(len(self.service.starts), 1)

    def test_nonzero_command_result_is_not_a_success_receipt(self):
        self.service.failure = 7
        self.assertEqual(self.control.start(self.identity())['phase'], 'UNKNOWN')
        self.assertIsNone(self.control.operations()[0]['receipt'])
        self.assertFalse(self.control.start(self.identity())['startIssued'])
        self.assertEqual(len(self.service.starts), 1)

    def test_failed_receipt_persistence_preserves_the_original_unknown(self):
        write = self.control.store.write
        def fail_receipt(name, value, new=False):
            if isinstance(value, dict) and value.get('receipt'):
                raise OSError('injected receipt storage failure')
            write(name, value, new=new)
        with patch.object(self.control.store, 'write', side_effect=fail_receipt):
            with self.assertRaises(OSError):
                self.control.start(self.identity())
        self.assertEqual(self.control.operations()[0]['phase'], 'UNKNOWN')
        self.assertFalse(self.control.start(self.identity())['startIssued'])
        self.assertEqual(len(self.service.starts), 1)

    def test_missing_registered_operation_cannot_be_treated_as_a_clean_host(self):
        identity = self.identity()
        self.control.start(identity)
        (self.control.store.directory / 'operations' / (identity + '.json')).unlink()
        with self.assertRaises(RuntimeError):
            self.control.snapshot()
        with self.assertRaises(RuntimeError):
            self.control.start(self.identity())
        self.assertEqual(len(self.service.starts), 1)

    def test_misbound_receipt_and_modified_intent_are_rejected(self):
        identity = self.identity()
        self.control.start(identity)
        path = self.control.store.directory / 'operations' / (identity + '.json')
        value = json.loads(path.read_text())
        value['receipt']['operationId'] = self.identity()
        path.write_text(json.dumps(value))
        with self.assertRaises(RuntimeError):
            self.control.operations()
        value['receipt']['operationId'] = identity
        value['intent']['command'] = ['/usr/bin/systemctl', 'restart', 'cockpit.socket']
        path.write_text(json.dumps(value))
        with self.assertRaises(RuntimeError):
            self.control.start(self.identity())
        self.assertEqual(len(self.service.starts), 1)

    def test_busy_systemd_job_is_rejected_before_a_start_intent(self):
        self.service.job = '123 /org/freedesktop/systemd1/job/123'
        result = self.control.start(self.identity())
        self.assertEqual(result['phase'], 'REJECTED_BEFORE_START')
        self.assertFalse(result['startIssued'])
        self.assertEqual(self.control.operations(), [])
        self.assertEqual(self.service.starts, [])

    def test_combined_certificate_file_exports_only_canonical_public_pem(self):
        secret = (FIXTURES / 'server-one.key').read_text()
        self.certificate.write_text(secret + self.pem)
        self.certificate.chmod(0o600)
        (self.certdir / 'only-private.key').write_text(secret)
        self.service.active = True
        value = self.control.snapshot()
        exported = value['certificate']['publicPem']
        self.assertEqual(ssl.PEM_cert_to_DER_cert(exported), self.der)
        self.assertEqual(exported, ssl.DER_cert_to_PEM_cert(self.der))
        self.assertNotIn('PRIVATE KEY', json.dumps(value))
        self.assertNotIn(secret.strip(), json.dumps(value))
        self.assertEqual(self.service.starts, [])

    def test_wrong_peer_and_unsafe_certificate_never_enable_the_entry(self):
        self.service.active = True
        self.control.peer = lambda: b'another peer'
        self.assertIsNone(self.control.snapshot()['certificate'])
        self.control.peer = lambda: self.der
        self.certificate.chmod(0o666)
        self.assertIsNone(self.control.snapshot()['certificate'])
        self.certificate.chmod(0o644)
        self.certificate.unlink()
        self.certificate.symlink_to(FIXTURES / 'server-one.pem')
        self.assertIsNone(self.control.snapshot()['certificate'])

    def test_authorized_public_copy_is_bound_to_original_file_peer_and_pem(self):
        self.service.active = True
        certificate = self.control.snapshot()['certificate']
        with patch.object(self.control, 'certificate_files', side_effect=PermissionError):
            self.assertTrue(self.control.snapshot()['certificateAuthorizationRequired'])
            self.assertEqual(self.control.snapshot(certificate)['certificate']['sha256'], certificate['sha256'])
            self.assertFalse(self.control.snapshot(certificate)['certificateAuthorizationRequired'])
            poisoned = dict(certificate, publicPem=certificate['publicPem'] + 'PRIVATE KEY')
            self.assertIsNone(self.control.snapshot(poisoned)['certificate'])
            poisoned = dict(certificate, path='../../a.cert')
            self.assertIsNone(self.control.snapshot(poisoned)['certificate'])
            self.certificate.write_text(self.pem + '\n')
            changed = self.control.snapshot(certificate)
            self.assertIsNone(changed['certificate'])
            self.assertTrue(changed['certificateAuthorizationRequired'])


class LocalRequestTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.directory = Path(self.temporary.name) / 'requests'
        self.requests = host_ui.UserRequests(self.directory)

    def receipt(self, identity):
        intent_sha = 'a' * 64
        return dict(operationId=identity, phase='APPLIED', intentSha256=intent_sha,
                    receipt=dict(operationId=identity, intentSha256=intent_sha,
                                 command=host_ui.START, returnCode=0, completedAt=host_ui.now()))

    def test_intent_is_durable_before_authorization_and_cannot_be_bypassed_by_reopening(self):
        identity = self.requests.begin([])
        self.assertEqual(self.requests.read()[0]['operationId'], identity)
        self.assertEqual(self.requests.read()[0]['transportOutcome'], 'INTENT')
        with self.assertRaises(RuntimeError):
            host_ui.UserRequests(self.directory).begin([])
        with self.assertRaises(RuntimeError):
            self.requests.begin([dict(operationId=identity, phase='APPLIED', receipt={'returnCode': 0})])
        self.assertEqual(len(self.requests.read()), 1)

    def test_only_the_matching_success_receipt_allows_a_later_explicit_request(self):
        identity = self.requests.begin([])
        another = self.receipt(str(uuid.uuid4()))
        with self.assertRaises(RuntimeError):
            self.requests.begin([another])
        new_identity = self.requests.begin([self.receipt(identity)])
        self.assertNotEqual(identity, new_identity)
        self.assertEqual(self.requests.read()[0]['transportOutcome'], 'INTENT')
        self.assertEqual(len(self.requests.read()), 2)

    def test_known_authorization_cancellation_keeps_history_and_permits_explicit_new_authorization(self):
        identity = self.requests.begin([])
        self.requests.finish(identity, 126, None, [], helper_unchanged=True)
        self.assertEqual(self.requests.read()[0]['transportOutcome'], 'AUTHORIZATION_NOT_GRANTED')
        self.assertNotEqual(self.requests.begin([]), identity)
        self.assertEqual(len(self.requests.read()), 2)

    def test_existing_root_intent_or_changed_helper_prevents_cancellation_inference(self):
        identity = self.requests.begin([])
        original = dict(operationId=identity, phase='UNKNOWN', receipt=None)
        self.requests.finish(identity, 126, None, [original], helper_unchanged=True)
        self.assertEqual(self.requests.read()[0]['transportOutcome'], 'UNKNOWN')
        self.requests.finish(identity, 127, None, [], helper_unchanged=False)
        with self.assertRaises(RuntimeError):
            self.requests.begin([])

    def test_transport_success_without_original_receipt_cannot_release_the_request(self):
        identity = self.requests.begin([])
        self.requests.finish(identity, 0, dict(operationId=identity, startIssued=True, phase='APPLIED'),
                             [dict(operationId=identity, phase='UNKNOWN', receipt=None)], helper_unchanged=True)
        self.assertEqual(self.requests.read()[0]['transportOutcome'], 'UNKNOWN')
        with self.assertRaises(RuntimeError):
            self.requests.begin([])

    def test_root_unknown_blocks_even_a_fresh_local_profile(self):
        with self.assertRaises(RuntimeError):
            self.requests.begin([dict(operationId=str(uuid.uuid4()), phase='UNKNOWN', receipt=None)])
        self.assertEqual(self.requests.read(), [])

    def test_missing_corrupt_or_unsafe_local_records_are_not_reset(self):
        self.requests.begin([])
        file = self.directory / 'requests.json'
        contents = file.read_bytes()
        file.unlink()
        with self.assertRaises(RuntimeError):
            self.requests.begin([])
        file.write_bytes(b'{broken')
        file.chmod(0o600)
        with self.assertRaises(ValueError):
            self.requests.begin([])
        file.write_bytes(contents)
        file.chmod(0o644)
        with self.assertRaises(RuntimeError):
            self.requests.begin([])
        self.assertEqual(file.read_bytes(), contents)


class ActualTlsCertificateTests(unittest.TestCase):
    def test_real_tls_probe_sends_no_http_or_credentials(self):
        listener = socket.socket()
        listener.bind(('127.0.0.1', 0))
        listener.listen(1)
        listener.settimeout(5)
        self.addCleanup(listener.close)
        port = listener.getsockname()[1]
        context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
        # The certificate-only client closes as soon as the handshake has
        # supplied the leaf. Keep post-handshake tickets out of this fixture.
        context.num_tickets = 0
        context.load_cert_chain(FIXTURES / 'server-one.pem', FIXTURES / 'server-one.key')
        received, errors = [], []
        def serve():
            try:
                raw, _ = listener.accept()
                with context.wrap_socket(raw, server_side=True) as connection:
                    connection.settimeout(5)
                    received.append(connection.recv(1))
            except Exception as exc:
                errors.append(exc)
        worker = threading.Thread(target=serve, daemon=True)
        worker.start()
        connect = socket.create_connection
        addresses = []
        def fixed_loopback(address, **kwargs):
            addresses.append(address)
            if address[0] == '::1':
                raise OSError(errno.EAFNOSUPPORT, 'IPv6 disabled in this independent fixture')
            return connect(('127.0.0.1', port), **kwargs)
        with patch.object(host_ui.socket, 'create_connection', side_effect=fixed_loopback):
            certificate = host_ui.loopback_certificate()
        worker.join(timeout=6)
        self.assertFalse(worker.is_alive())
        self.assertEqual(errors, [])
        self.assertEqual(received, [b''])
        self.assertEqual(addresses, [('127.0.0.1', 9090), ('::1', 9090)])
        self.assertEqual(certificate, ssl.PEM_cert_to_DER_cert((FIXTURES / 'server-one.pem').read_text()))

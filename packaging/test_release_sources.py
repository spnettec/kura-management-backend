#!/usr/bin/env python3
# Copyright (c) 2026 YOFC IOT and others
# SPDX-License-Identifier: EPL-2.0
"""The release source manifest binds every packaged source before deb creation."""
import copy
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import build


class ReleaseSourceTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.source = (Path(temporary.name) / 'source').resolve()
        self.manifest = build.stage(self.source)

    def test_self_contained_export_can_verify_its_own_complete_source_set(self):
        with patch.object(build, 'ROOT', self.source):
            build.verify_source_manifest(self.manifest)

    def test_changed_privileged_code_rust_and_web_source_are_refused(self):
        for relative in ('packaging/lifecycle.py', 'crates/console/src/main.rs', 'frontend/react/src/main.tsx'):
            with self.subTest(source=relative), patch.object(build, 'ROOT', self.source):
                path = self.source / relative
                original = path.read_bytes()
                path.write_bytes(original + b'\n// source cohort drift\n')
                with self.assertRaisesRegex(RuntimeError, 'differs from staged'):
                    build.verify_source_manifest(self.manifest)
                path.write_bytes(original)

    def test_omitted_or_duplicated_source_record_is_refused(self):
        for rows in (self.manifest['sourceFiles'][1:], self.manifest['sourceFiles'] + self.manifest['sourceFiles'][:1]):
            manifest = copy.deepcopy(self.manifest)
            manifest['sourceFiles'] = rows
            with patch.object(build, 'ROOT', self.source), self.assertRaises(RuntimeError):
                build.verify_source_manifest(manifest)

    def test_source_file_and_parent_symlinks_are_refused(self):
        path = self.source / 'crates/console/src/main.rs'
        real = self.source / 'saved-main.rs'
        path.rename(real)
        path.symlink_to(real)
        with patch.object(build, 'ROOT', self.source), self.assertRaisesRegex(RuntimeError, 'Source symlinks'):
            build.verify_source_manifest(self.manifest)
        path.unlink()
        real.rename(path)
        directory = self.source / 'frontend/react/src'
        outside = self.source / 'saved-web-src'
        directory.rename(outside)
        directory.symlink_to(outside, target_is_directory=True)
        with patch.object(build, 'ROOT', self.source), self.assertRaisesRegex(RuntimeError, 'Source symlinks'):
            build.verify_source_manifest(self.manifest)


if __name__ == '__main__':
    unittest.main()

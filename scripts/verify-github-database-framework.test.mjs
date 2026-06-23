#!/usr/bin/env node
import assert from 'node:assert/strict';
import { validateDatabaseFramework } from '../../sdkwork-specs/tools/check-database-framework-standard.mjs';

const result = validateDatabaseFramework(process.cwd());
assert.equal(result.skipped, false, 'sdkwork-github must own database/');
assert.equal(
  result.ok,
  true,
  `database framework validation failed:\n${result.failures.map((item) => `- ${item}`).join('\n')}`,
);

process.stdout.write('verify-github-database-framework.test.mjs passed\n');

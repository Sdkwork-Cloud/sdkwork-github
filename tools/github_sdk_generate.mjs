#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const appApiPath = 'apis/app-api/github/github-app-api.openapi.json';

function run(script, args = []) {
  const result = spawnSync('node', [path.join(workspaceRoot, script), ...args], {
    cwd: workspaceRoot,
    stdio: 'inherit',
  });
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

const check = process.argv.includes('--check');

run('tools/github_api_materialize.mjs', check ? ['--check'] : []);

if (!check) {
  run('sdks/sdkwork-github-app-sdk/bin/generate-sdk.mjs', [
    '--input',
    appApiPath,
    '--language',
    'typescript',
  ]);
}

process.stdout.write(`[github_sdk_generate] ${check ? 'check passed' : 'generation completed'}\n`);

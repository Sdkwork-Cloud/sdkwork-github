#!/usr/bin/env node
import { spawn } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const profile = resolve(root, 'configs/topology/standalone.unified-process.development.env');
const env = Object.fromEntries(
  readFileSync(profile, 'utf8')
    .split(/\r?\n/u)
    .map((line) => line.trim())
    .filter((line) => line && !line.startsWith('#'))
    .map((line) => {
      const index = line.indexOf('=');
      return [line.slice(0, index), line.slice(index + 1)];
    }),
);

const api = spawn('cargo', ['run', '-p', 'sdkwork-github-api-server'], {
  cwd: root,
  env: { ...process.env, ...env },
  stdio: 'inherit',
  shell: true,
});

const web = spawn('pnpm', ['--dir', 'apps/sdkwork-github-pc', 'dev'], {
  cwd: root,
  env: { ...process.env, ...env },
  stdio: 'inherit',
  shell: true,
});

function shutdown() {
  api.kill();
  web.kill();
  process.exit(0);
}

process.on('SIGINT', shutdown);
process.on('SIGTERM', shutdown);

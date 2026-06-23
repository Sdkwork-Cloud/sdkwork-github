#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(__dirname, '..');

const required = [
  'apis/app-api/github/github-app-api.openapi.json',
  'sdks/_route-manifests/app-api/sdkwork-router-github-app-api.route-manifest.json',
];

const check = process.argv.includes('--check');
let failed = false;

for (const relativePath of required) {
  const absolutePath = resolve(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    console.error(`missing required artifact: ${relativePath}`);
    failed = true;
    continue;
  }
  if (relativePath.endsWith('.openapi.json')) {
    const openapi = JSON.parse(readFileSync(absolutePath, 'utf8'));
    if (openapi.openapi !== '3.1.2') {
      console.error(`${relativePath} must declare openapi 3.1.2`);
      failed = true;
    }
  }
}

if (failed) {
  process.exit(1);
}

console.log(check ? 'github sdk contract check passed' : 'github sdk contract validated');

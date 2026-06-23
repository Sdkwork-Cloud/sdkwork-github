#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(__dirname, '..');

const OPENAPI_PATH = 'apis/app-api/github/github-app-api.openapi.json';
const MANIFEST_PATH =
  'sdks/_route-manifests/app-api/sdkwork-router-github-app-api.route-manifest.json';

const check = process.argv.includes('--check');

function readJson(relativePath) {
  return JSON.parse(readFileSync(resolve(repoRoot, relativePath), 'utf8'));
}

function listOpenApiOperations(openapi) {
  const operations = [];
  for (const [path, pathItem] of Object.entries(openapi.paths ?? {})) {
    for (const [method, operation] of Object.entries(pathItem ?? {})) {
      if (!['get', 'post', 'put', 'patch', 'delete'].includes(method)) {
        continue;
      }
      operations.push({
        method: method.toUpperCase(),
        path,
        operationId: operation.operationId,
        requestContext: operation['x-sdkwork-request-context'],
        apiSurface: operation['x-sdkwork-api-surface'],
      });
    }
  }
  return operations;
}

function main() {
  for (const relativePath of [OPENAPI_PATH, MANIFEST_PATH]) {
    if (!existsSync(resolve(repoRoot, relativePath))) {
      console.error(`missing required artifact: ${relativePath}`);
      process.exit(1);
    }
  }

  const openapi = readJson(OPENAPI_PATH);
  const manifest = readJson(MANIFEST_PATH);
  const openapiOps = listOpenApiOperations(openapi);
  const manifestRoutes = manifest.routes ?? [];

  const failures = [];
  if (openapi.openapi !== '3.1.2') {
    failures.push(`${OPENAPI_PATH} must declare openapi 3.1.2`);
  }

  for (const operation of openapiOps) {
    if (operation.requestContext !== 'WebRequestContext') {
      failures.push(`${operation.operationId} must declare x-sdkwork-request-context=WebRequestContext`);
    }
    if (operation.apiSurface !== 'app-api') {
      failures.push(`${operation.operationId} must declare x-sdkwork-api-surface=app-api`);
    }
  }

  const manifestKeys = new Set(
    manifestRoutes.map((route) => `${route.method} ${route.path}#${route.operationId}`),
  );
  for (const operation of openapiOps) {
    const key = `${operation.method} ${operation.path}#${operation.operationId}`;
    if (!manifestKeys.has(key)) {
      failures.push(`route manifest missing operation ${key}`);
    }
  }

  const openapiKeys = new Set(
    openapiOps.map((operation) => `${operation.method} ${operation.path}#${operation.operationId}`),
  );
  for (const route of manifestRoutes) {
    const key = `${route.method} ${route.path}#${route.operationId}`;
    if (!openapiKeys.has(key)) {
      failures.push(`OpenAPI missing operation ${key}`);
    }
  }

  if (failures.length > 0) {
    console.error(failures.map((item) => `- ${item}`).join('\n'));
    process.exit(1);
  }

  console.log(check ? 'github api materialize check passed' : 'github api materialize validated');
}

main();

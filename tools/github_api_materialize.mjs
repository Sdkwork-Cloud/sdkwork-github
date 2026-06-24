#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(__dirname, '..');

const SURFACES = [
  {
    label: 'app-api',
    openapiPath: 'apis/app-api/github/github-app-api.openapi.json',
    manifestPath:
      'sdks/_route-manifests/app-api/sdkwork-router-github-app-api.route-manifest.json',
    expectedApiSurface: 'app-api',
  },
  {
    label: 'backend-api',
    openapiPath: 'apis/backend-api/github/github-backend-api.openapi.json',
    manifestPath:
      'sdks/_route-manifests/backend-api/sdkwork-router-github-backend-api.route-manifest.json',
    expectedApiSurface: 'backend-api',
  },
];

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

function validateSurface({ label, openapiPath, manifestPath, expectedApiSurface }) {
  const failures = [];
  for (const relativePath of [openapiPath, manifestPath]) {
    if (!existsSync(resolve(repoRoot, relativePath))) {
      failures.push(`${relativePath} should exist`);
    }
  }
  if (failures.length > 0) {
    return failures;
  }

  const openapi = readJson(openapiPath);
  const manifest = readJson(manifestPath);
  const openapiOps = listOpenApiOperations(openapi);
  const manifestRoutes = manifest.routes ?? [];

  if (openapi.openapi !== '3.1.2') {
    failures.push(`${openapiPath} must declare openapi 3.1.2`);
  }

  for (const operation of openapiOps) {
    if (operation.requestContext !== 'WebRequestContext') {
      failures.push(
        `${label} ${operation.operationId} must declare x-sdkwork-request-context=WebRequestContext`,
      );
    }
    if (operation.apiSurface !== expectedApiSurface) {
      failures.push(
        `${label} ${operation.operationId} must declare x-sdkwork-api-surface=${expectedApiSurface}`,
      );
    }
  }

  const manifestKeys = new Set(
    manifestRoutes.map((route) => `${route.method} ${route.path}#${route.operationId}`),
  );
  for (const operation of openapiOps) {
    const key = `${operation.method} ${operation.path}#${operation.operationId}`;
    if (!manifestKeys.has(key)) {
      failures.push(`${label} route manifest missing operation ${key}`);
    }
  }

  const openapiKeys = new Set(
    openapiOps.map((operation) => `${operation.method} ${operation.path}#${operation.operationId}`),
  );
  for (const route of manifestRoutes) {
    const key = `${route.method} ${route.path}#${route.operationId}`;
    if (!openapiKeys.has(key)) {
      failures.push(`${label} OpenAPI missing operation ${key}`);
    }
    if (route.requestContext !== 'WebRequestContext') {
      failures.push(`${label} manifest route ${key} must declare requestContext=WebRequestContext`);
    }
    if (route.apiSurface !== expectedApiSurface) {
      failures.push(`${label} manifest route ${key} must declare apiSurface=${expectedApiSurface}`);
    }
  }

  return failures;
}

function main() {
  const failures = SURFACES.flatMap((surface) => validateSurface(surface));
  if (failures.length > 0) {
    console.error(failures.map((item) => `- ${item}`).join('\n'));
    process.exit(1);
  }

  console.log(check ? 'github api materialize check passed' : 'github api materialize validated');
}

main();

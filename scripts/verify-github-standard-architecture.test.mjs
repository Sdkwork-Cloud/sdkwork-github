import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

const ROOT = process.cwd();

const STANDARD_ROOT_DIRECTORIES = [
  'apis', 'apps', 'crates', 'database', 'sdks', 'jobs', 'tools', 'plugins',
  'examples', 'configs', 'deployments', 'scripts', 'docs', 'tests',
];

const REQUIRED_WORKSPACE_FILES = [
  'AGENTS.md', 'CLAUDE.md', 'CODEX.md', 'GEMINI.md', 'README.md', 'Cargo.toml',
  'sdkwork.workflow.json', '.github/workflows/package.yml',
  '.sdkwork/README.md', '.sdkwork/skills/README.md', '.sdkwork/plugins/README.md',
  'docs/root-layout.md', 'sdkwork.app.config.json',
];

const API_INPUTS = [
  'apis/app-api/github/github-app-api.openapi.json',
  'apis/backend-api/github/github-backend-api.openapi.json',
  'apis/open-api/github/github-open-api.openapi.json',
];

const WEB_FRAMEWORK_CRATES = [
  'crates/sdkwork-router-github-app-api/Cargo.toml',
  'crates/sdkwork-router-github-backend-api/Cargo.toml',
  'crates/sdkwork-github-api-server/Cargo.toml',
];

function read(relativePath) {
  return readFileSync(path.join(ROOT, relativePath), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath).replace(/^\uFEFF/u, ''));
}

function exists(relativePath) {
  return existsSync(path.join(ROOT, relativePath));
}

test('declares SDKWork standard root directory dictionary', () => {
  for (const directory of STANDARD_ROOT_DIRECTORIES) {
    assert.equal(exists(directory), true, `${directory}/ should exist`);
    assert.equal(exists(path.join(directory, 'README.md')), true, `${directory}/README.md should exist`);
  }
});

test('declares workspace agent entrypoints and packaging workflow', () => {
  for (const file of REQUIRED_WORKSPACE_FILES) {
    assert.equal(exists(file), true, `${file} should exist`);
  }
  const workflow = readJson('sdkwork.workflow.json');
  assert.equal(workflow.app.id, 'sdkwork-github');
});

test('declares author-owned API contracts under apis/', () => {
  for (const file of API_INPUTS) {
    assert.equal(exists(file), true, `${file} should exist`);
    assert.equal(readJson(file).openapi, '3.1.2');
  }
});

test('integrates sdkwork-web-framework in HTTP route crates and api-server', () => {
  const rootCargo = read('Cargo.toml');
  assert.match(rootCargo, /sdkwork-web-core/);
  assert.match(rootCargo, /sdkwork-web-axum/);
  for (const cargoPath of WEB_FRAMEWORK_CRATES) {
    assert.match(read(cargoPath), /sdkwork-web-/);
  }
  assert.match(read('crates/sdkwork-github-api-server/src/bootstrap/auth.rs'), /wrap_router_with_web_framework_from_env/);
});

test('integrates sdkwork-utils in Rust crates and PC commons', () => {
  assert.match(read('Cargo.toml'), /sdkwork-utils-rust/);
  assert.match(read('crates/sdkwork-router-github-app-api/Cargo.toml'), /sdkwork-utils-rust/);
  assert.match(read('crates/sdkwork-github-integration-service/src/service.rs'), /sdkwork_utils_rust::string::is_blank/);
  assert.match(read('apps/sdkwork-github-pc/pnpm-workspace.yaml'), /sdkwork-utils-typescript/);
  assert.match(read('apps/sdkwork-github-pc/packages/sdkwork-github-pc-commons/src/utils/text.ts'), /@sdkwork\/utils/);
});

test('integrates sdkwork-database in api-server bootstrap', () => {
  assert.match(read('crates/sdkwork-github-api-server/Cargo.toml'), /sdkwork-database-config/);
  assert.match(read('crates/sdkwork-github-api-server/src/bootstrap/database.rs'), /DatabaseConfig::from_env\("github"\)/);
  assert.equal(exists('database/database.manifest.json'), true);
});

test('does not declare sdkwork-discovery without RPC services', () => {
  assert.doesNotMatch(read('Cargo.toml'), /sdkwork-discovery/);
  assert.equal(exists('apis/rpc'), false);
});

test('route manifest declares WebRequestContext and apiSurface', () => {
  const manifest = readJson('sdks/_route-manifests/app-api/sdkwork-router-github-app-api.route-manifest.json');
  for (const route of manifest.routes) {
    assert.equal(route.requestContext, 'WebRequestContext');
    assert.equal(route.apiSurface, 'app-api');
  }
});

test('OpenAPI app-api declares x-sdkwork metadata on operations', () => {
  const openapi = readJson('apis/app-api/github/github-app-api.openapi.json');
  let count = 0;
  for (const pathItem of Object.values(openapi.paths ?? {})) {
    for (const [method, operation] of Object.entries(pathItem ?? {})) {
      if (!['get', 'post', 'put', 'patch', 'delete'].includes(method)) continue;
      count += 1;
      assert.equal(operation['x-sdkwork-request-context'], 'WebRequestContext');
      assert.equal(operation['x-sdkwork-api-surface'], 'app-api');
    }
  }
  assert.ok(count > 0);
});

test('PC application root follows apps/sdkwork-github-pc layout', () => {
  assert.equal(exists('apps/sdkwork-github-pc/sdkwork.app.config.json'), true);
  assert.equal(exists('apps/sdkwork-github-pc/AGENTS.md'), true);
  const manifest = readJson('apps/sdkwork-github-pc/sdkwork.app.config.json');
  assert.equal(manifest.kind, 'sdkwork.app');
});

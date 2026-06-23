#!/usr/bin/env node
import {
  resolveFamilySdkRoot,
  runGithubSdkGenerator,
} from '../../../tools/github_sdk_generator_runner.mjs';

runGithubSdkGenerator(
  {
    apiAuthority: 'sdkwork-github-app-api',
    apiPrefix: '/app/v3/api',
    defaultBaseUrl: 'http://127.0.0.1:4100',
    defaultOpenapiFile: 'github-app-api.openapi.json',
    sdkName: 'sdkwork-github-app-sdk',
    sdkRoot: resolveFamilySdkRoot(import.meta.url),
    sdkType: 'app',
  },
  process.argv.slice(2),
);

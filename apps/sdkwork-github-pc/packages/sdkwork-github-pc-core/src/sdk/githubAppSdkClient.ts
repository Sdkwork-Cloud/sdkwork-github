import { createClient, type SdkworkAppClient } from '@sdkwork/github-app-sdk';
import type { GithubRuntimeConfig } from '../config/runtimeConfig';
import { normalizeGeneratedSdkBaseUrl } from '../config/runtimeConfig';
import type { GithubSessionTokenManager } from '../session/sessionTokenManager';

const APP_API_PREFIX = '/app/v3/api';

export interface GithubAppSdkClient {
  client: SdkworkAppClient;
  setTokenManager(manager: GithubSessionTokenManager): void;
}

export function createGithubAppSdkClient({
  config,
  tokenManager,
  sdkClient,
}: {
  config: GithubRuntimeConfig;
  tokenManager: GithubSessionTokenManager;
  sdkClient?: SdkworkAppClient;
}): GithubAppSdkClient {
  const generatedClient =
    sdkClient
    ?? createClient({
      authMode: 'dual-token',
      baseUrl: normalizeGeneratedSdkBaseUrl(config.appApiBaseUrl, APP_API_PREFIX),
      tokenManager: tokenManager as never,
    });
  generatedClient.setTokenManager(tokenManager as never);

  return {
    client: generatedClient,
    setTokenManager(manager) {
      generatedClient.setTokenManager(manager as never);
    },
  };
}

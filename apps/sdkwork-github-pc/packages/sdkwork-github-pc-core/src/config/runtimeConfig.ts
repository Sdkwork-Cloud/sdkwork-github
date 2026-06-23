export type SdkworkEnvironment = 'development' | 'test' | 'staging' | 'production';
export type SdkworkDeploymentProfile = 'standalone' | 'cloud';

export interface GithubRuntimeConfig {
  deploymentProfile: SdkworkDeploymentProfile;
  environment: SdkworkEnvironment;
  appKey: 'sdkwork-github-pc';
  appApiBaseUrl: string;
  auth: {
    tokenStorage: 'browser-session' | 'browser-local' | 'memory';
  };
}

export interface RuntimeEnv {
  VITE_SDKWORK_GITHUB_DEPLOYMENT_PROFILE?: string;
  VITE_SDKWORK_GITHUB_ENVIRONMENT?: string;
  VITE_SDKWORK_GITHUB_APPLICATION_PUBLIC_HTTP_URL?: string;
  DEV?: boolean;
}

const LOCAL_APP_API_BASE_URL = 'http://127.0.0.1:4100';

function readEnv(env: RuntimeEnv, key: string): string | undefined {
  const value = env[key as keyof RuntimeEnv];
  return typeof value === 'string' && value.trim() ? value.trim() : undefined;
}

export function createRuntimeConfig(env: RuntimeEnv): GithubRuntimeConfig {
  const deploymentProfile =
    readEnv(env, 'VITE_SDKWORK_GITHUB_DEPLOYMENT_PROFILE') === 'cloud' ? 'cloud' : 'standalone';
  const environmentRaw = readEnv(env, 'VITE_SDKWORK_GITHUB_ENVIRONMENT') ?? 'development';
  const environment = ['development', 'test', 'staging', 'production'].includes(environmentRaw)
    ? (environmentRaw as SdkworkEnvironment)
    : 'development';

  const configuredBaseUrl = readEnv(env, 'VITE_SDKWORK_GITHUB_APPLICATION_PUBLIC_HTTP_URL');
  const appApiBaseUrl =
    configuredBaseUrl
    ?? (env.DEV ? LOCAL_APP_API_BASE_URL : (typeof window !== 'undefined' ? window.location.origin : LOCAL_APP_API_BASE_URL));

  return {
    appKey: 'sdkwork-github-pc',
    appApiBaseUrl,
    auth: { tokenStorage: 'browser-session' },
    deploymentProfile,
    environment,
  };
}

export function normalizeGeneratedSdkBaseUrl(baseUrl: string, apiPrefix = '/app/v3/api'): string {
  const normalizedBaseUrl = baseUrl.replace(/\/+$/, '');
  const normalizedApiPrefix = apiPrefix.replace(/\/+$/, '');
  if (normalizedBaseUrl.endsWith(normalizedApiPrefix)) {
    return normalizedBaseUrl.slice(0, -normalizedApiPrefix.length) || normalizedBaseUrl;
  }
  return normalizedBaseUrl;
}

export interface SdkworkAppbasePcAuthRuntimeComposition {
  getRuntime(): unknown;
  runtime: unknown;
}

export interface SdkworkAppbasePcAuthRuntimeOptions {
  app: {
    appId: string;
    deploymentMode: 'saas' | 'local';
    environment: 'prod' | 'dev';
    platform: string;
  };
  baseUrls: {
    appbaseAppApiBaseUrl: string;
  };
  createAppbaseAppClient: () => unknown;
  sdkClients: unknown[];
  sessionBridge: {
    clearSession: () => void;
    commitSession: (iamSession: unknown) => unknown;
    readSession: () => unknown;
  };
  tokenManager: unknown;
}

export function createSdkworkAppbasePcAuthRuntime(
  options: SdkworkAppbasePcAuthRuntimeOptions,
): SdkworkAppbasePcAuthRuntimeComposition;

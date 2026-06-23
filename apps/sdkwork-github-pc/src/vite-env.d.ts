/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_SDKWORK_GITHUB_DEPLOYMENT_PROFILE?: string;
  readonly VITE_SDKWORK_GITHUB_ENVIRONMENT?: string;
  readonly VITE_SDKWORK_GITHUB_APPLICATION_PUBLIC_HTTP_URL?: string;
  readonly DEV: boolean;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}

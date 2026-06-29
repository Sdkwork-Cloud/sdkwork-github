import type { ReactElement, ReactNode } from 'react';

export interface SdkworkAuthRuntimeConfig {
  loginMethods?: string[];
  qrLoginEnabled?: boolean;
  registerMethods?: string[];
  verificationPolicy?: Record<string, boolean>;
}

export interface SdkworkIamAuthRoutesProps {
  basePath?: string;
  getRuntime: () => unknown;
  runtimeConfig?: SdkworkAuthRuntimeConfig;
}

export function SdkworkIamAuthRoutes(
  props: SdkworkIamAuthRoutesProps,
): ReactElement | null;

export interface SdkworkSessionAuthBrowserRootProps {
  children?: ReactNode;
}

export function SdkworkSessionAuthBrowserRoot(
  props: SdkworkSessionAuthBrowserRootProps,
): ReactElement | null;

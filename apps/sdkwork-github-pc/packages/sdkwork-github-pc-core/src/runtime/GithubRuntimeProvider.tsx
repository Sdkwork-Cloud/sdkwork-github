import React, { createContext, useContext } from 'react';
import type { GithubIamRuntime } from '../iam/githubIamRuntime';
import type { GithubRuntimeConfig } from '../config/runtimeConfig';
import type { GithubAppSdkClient } from '../sdk/githubAppSdkClient';
import type { SessionStore } from '../session/sessionStore';

export interface GithubPcRuntime {
  config: GithubRuntimeConfig;
  session: SessionStore;
  iamRuntime: GithubIamRuntime;
  githubSdk: GithubAppSdkClient;
}

const GithubRuntimeContext = createContext<GithubPcRuntime | null>(null);

export function GithubRuntimeProvider({
  runtime,
  children,
}: {
  runtime: GithubPcRuntime;
  children: React.ReactNode;
}) {
  return React.createElement(GithubRuntimeContext.Provider, { value: runtime }, children);
}

export function useGithubPcRuntime(): GithubPcRuntime {
  const runtime = useContext(GithubRuntimeContext);
  if (!runtime) {
    throw new Error('GithubRuntimeProvider is required');
  }
  return runtime;
}

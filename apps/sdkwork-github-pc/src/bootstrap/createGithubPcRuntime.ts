import {
  createGithubAppSdkClient,
  createGithubIamRuntime,
  createGithubSessionTokenManager,
  createRuntimeConfig,
  createSessionStore,
  type GithubPcRuntime,
} from '@sdkwork/github-pc-core';

export function createGithubPcRuntime(): GithubPcRuntime {
  const config = createRuntimeConfig(import.meta.env);
  const session = createSessionStore(
    typeof window !== 'undefined' ? window.sessionStorage : undefined,
  );
  const tokenManager = createGithubSessionTokenManager(session);
  const githubSdk = createGithubAppSdkClient({ config, tokenManager });
  const iamRuntime = createGithubIamRuntime({
    config,
    githubSdk,
    session,
    tokenManager,
  });

  return {
    config,
    githubSdk,
    iamRuntime,
    session,
  };
}

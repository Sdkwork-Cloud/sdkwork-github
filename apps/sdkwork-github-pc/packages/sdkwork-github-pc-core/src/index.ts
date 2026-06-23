export const APP_TITLE = 'SDKWork GitHub';

export { createRuntimeConfig, normalizeGeneratedSdkBaseUrl } from './config/runtimeConfig';
export type { GithubRuntimeConfig } from './config/runtimeConfig';
export {
  createSessionStore,
  hasGithubIamSession,
} from './session/sessionStore';
export type { SessionSnapshot, SessionStore } from './session/sessionStore';
export { createGithubSessionTokenManager } from './session/sessionTokenManager';
export type { GithubSessionTokenManager } from './session/sessionTokenManager';
export { createGithubAppSdkClient } from './sdk/githubAppSdkClient';
export type { GithubAppSdkClient } from './sdk/githubAppSdkClient';
export { createGithubIamRuntime } from './iam/githubIamRuntime';
export type { GithubIamRuntime } from './iam/githubIamRuntime';
export { GithubAuthGate } from './auth/authGate';
export {
  GithubRuntimeProvider,
  useGithubPcRuntime,
} from './runtime/GithubRuntimeProvider';
export type { GithubPcRuntime } from './runtime/GithubRuntimeProvider';

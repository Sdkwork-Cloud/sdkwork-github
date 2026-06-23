import { createClient, type SdkworkAppClient } from '@sdkwork/appbase-app-sdk';
import {
  createSdkworkAppbasePcAuthRuntime,
  type SdkworkAppbasePcAuthRuntimeComposition,
} from '@sdkwork/auth-runtime-pc-react';
import type { IamAppContext } from '@sdkwork/iam-contracts';
import type { GithubRuntimeConfig } from '../config/runtimeConfig';
import { normalizeGeneratedSdkBaseUrl } from '../config/runtimeConfig';
import type { SessionSnapshot, SessionStore } from '../session/sessionStore';
import type { GithubSessionTokenManager } from '../session/sessionTokenManager';
import type { GithubAppSdkClient } from '../sdk/githubAppSdkClient';

const APP_API_PREFIX = '/app/v3/api';

export type GithubIamRuntime = ReturnType<SdkworkAppbasePcAuthRuntimeComposition['getRuntime']>;

interface IamSessionLike {
  accessToken?: string;
  authToken?: string;
  refreshToken?: string;
  sessionId?: string;
  context?: IamAppContext;
}

export function createGithubIamRuntime({
  config,
  session,
  tokenManager,
  githubSdk,
}: {
  config: GithubRuntimeConfig;
  session: SessionStore;
  tokenManager: GithubSessionTokenManager;
  githubSdk: GithubAppSdkClient;
}): GithubIamRuntime {
  const appbaseClient = createAppbaseAppClient(config, tokenManager);
  const composition = createSdkworkAppbasePcAuthRuntime({
    app: {
      appId: config.appKey,
      deploymentMode: config.deploymentProfile === 'cloud' ? 'saas' : 'local',
      environment: config.environment === 'production' ? 'prod' : 'dev',
      platform: 'pc',
    },
    baseUrls: {
      appbaseAppApiBaseUrl: config.appApiBaseUrl,
    },
    createAppbaseAppClient: () => appbaseClient,
    sdkClients: [githubSdk],
    sessionBridge: {
      clearSession: () => session.clearSession(),
      commitSession: (iamSession) => {
        commitIamSession(session, iamSession as IamSessionLike);
        return toBridgeSession(session.getSnapshot()) ?? undefined;
      },
      readSession: () => toBridgeSession(session.getSnapshot()),
    },
    tokenManager: tokenManager as never,
  });

  return composition.runtime;
}

function createAppbaseAppClient(
  config: GithubRuntimeConfig,
  tokenManager: GithubSessionTokenManager,
): SdkworkAppClient {
  return createClient({
    authMode: 'dual-token',
    baseUrl: normalizeGeneratedSdkBaseUrl(config.appApiBaseUrl, APP_API_PREFIX),
    tokenManager: tokenManager as never,
  });
}

function commitIamSession(session: SessionStore, iamSession: IamSessionLike): void {
  const next: SessionSnapshot = {
    ...session.getSnapshot(),
    accessToken: iamSession.accessToken,
    authToken: iamSession.authToken,
    refreshToken: iamSession.refreshToken,
    sessionId: iamSession.sessionId ?? iamSession.context?.sessionId,
    context: iamSession.context
      ? {
        tenantId: iamSession.context.tenantId,
        userId: iamSession.context.userId,
        organizationId: iamSession.context.organizationId,
        sessionId: iamSession.context.sessionId,
      }
      : undefined,
  };
  if (!next.authToken && !next.accessToken && !next.refreshToken) {
    session.clearSession();
    return;
  }
  session.setSession(next);
}

function toBridgeSession(snapshot: SessionSnapshot): IamSessionLike | null {
  if (!snapshot.authToken && !snapshot.accessToken && !snapshot.refreshToken) {
    return null;
  }
  return {
    accessToken: snapshot.accessToken,
    authToken: snapshot.authToken,
    refreshToken: snapshot.refreshToken,
    sessionId: snapshot.sessionId,
    context: snapshot.context?.tenantId
      ? {
        appId: 'sdkwork-github-pc',
        authLevel: 'password',
        dataScope: [],
        deploymentMode: 'local',
        environment: 'dev',
        organizationId: snapshot.context.organizationId,
        permissionScope: [],
        sessionId: snapshot.context.sessionId ?? snapshot.sessionId ?? 'session',
        tenantId: snapshot.context.tenantId,
        userId: snapshot.context.userId ?? 'user',
      }
      : undefined,
  };
}

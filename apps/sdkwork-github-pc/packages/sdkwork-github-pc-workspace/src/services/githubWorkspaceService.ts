import type { SdkworkAppClient } from '@sdkwork/github-app-sdk';
import type { SessionSnapshot } from '@sdkwork/github-pc-core';

function resolveScope(context: SessionSnapshot['context']) {
  if (!context?.tenantId) {
    throw new Error('tenant context is required');
  }
  return {
    tenantId: context.tenantId,
    organizationId: context.organizationId,
  };
}

export async function listRepositories(
  client: SdkworkAppClient,
  context: SessionSnapshot['context'],
  page = 1,
  pageSize = 20,
) {
  const scope = resolveScope(context);
  return client.github.repositories.list({
    ...scope,
    page,
    pageSize,
  });
}

export async function listIssues(
  client: SdkworkAppClient,
  context: SessionSnapshot['context'],
  repositoryId?: string,
  page = 1,
  pageSize = 20,
) {
  const scope = resolveScope(context);
  return client.github.issues.list({
    ...scope,
    repositoryId,
    page,
    pageSize,
  });
}

export async function syncRepositories(
  client: SdkworkAppClient,
  context: SessionSnapshot['context'],
) {
  const scope = resolveScope(context);
  return client.github.repositories.sync(scope);
}

export async function syncIssues(
  client: SdkworkAppClient,
  context: SessionSnapshot['context'],
  repositoryId?: string,
) {
  const scope = resolveScope(context);
  return client.github.issues.sync({
    ...scope,
    repositoryId,
  });
}

export async function getIntegrationStatus(
  client: SdkworkAppClient,
  context: SessionSnapshot['context'],
) {
  const scope = resolveScope(context);
  return client.github.integration.status(scope);
}

export async function linkIntegration(
  client: SdkworkAppClient,
  context: SessionSnapshot['context'],
  accessToken: string,
  externalAccountId?: string,
) {
  const scope = resolveScope(context);
  return client.github.integration.link(
    {
      access_token: accessToken,
      external_account_id: externalAccountId,
    },
    scope,
  );
}

export async function unlinkIntegration(
  client: SdkworkAppClient,
  context: SessionSnapshot['context'],
) {
  const scope = resolveScope(context);
  return client.github.integration.unlink(scope);
}

export async function beginOAuthIntegration(
  client: SdkworkAppClient,
  context: SessionSnapshot['context'],
) {
  const scope = resolveScope(context);
  return client.github.integration.oauth.begin(scope);
}

export async function listPlans(
  client: SdkworkAppClient,
  context: SessionSnapshot['context'],
  page = 1,
  pageSize = 20,
) {
  const scope = resolveScope(context);
  return client.github.plans.list({
    ...scope,
    page,
    pageSize,
  });
}

import { appApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { IntegrationStatus, IssuePage, LinkIntegrationRequest, PlanPage, RepositoryPage, SyncResult } from '../types';


export interface GithubIntegrationStatusParams {
  tenantId?: string;
  organizationId?: string;
}

export interface GithubIntegrationLinkParams {
  tenantId?: string;
  organizationId?: string;
}

export interface GithubIntegrationUnlinkParams {
  tenantId?: string;
  organizationId?: string;
}

export class GithubIntegrationApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Get GitHub integration link status. */
  async status(params?: GithubIntegrationStatusParams): Promise<IntegrationStatus> {
    const query = buildQueryString([
      { name: 'tenant_id', value: params?.tenantId, style: 'form', explode: true, allowReserved: false },
      { name: 'organization_id', value: params?.organizationId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<IntegrationStatus>(appendQueryString(appApiPath(`/github/integration`), query));
  }

/** Link GitHub integration credentials for the current tenant scope. */
  async link(body: LinkIntegrationRequest, params?: GithubIntegrationLinkParams): Promise<IntegrationStatus> {
    const query = buildQueryString([
      { name: 'tenant_id', value: params?.tenantId, style: 'form', explode: true, allowReserved: false },
      { name: 'organization_id', value: params?.organizationId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.post<IntegrationStatus>(appendQueryString(appApiPath(`/github/integration`), query), body, undefined, undefined, 'application/json');
  }

/** Revoke linked GitHub integration credentials. */
  async unlink(params?: GithubIntegrationUnlinkParams): Promise<IntegrationStatus> {
    const query = buildQueryString([
      { name: 'tenant_id', value: params?.tenantId, style: 'form', explode: true, allowReserved: false },
      { name: 'organization_id', value: params?.organizationId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.delete<IntegrationStatus>(appendQueryString(appApiPath(`/github/integration`), query));
  }
}

export interface GithubPlansListParams {
  tenantId?: string;
  organizationId?: string;
  page?: number;
  pageSize?: number;
}

export class GithubPlansApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List feature plans and checklists. */
  async list(params?: GithubPlansListParams): Promise<PlanPage> {
    const query = buildQueryString([
      { name: 'tenant_id', value: params?.tenantId, style: 'form', explode: true, allowReserved: false },
      { name: 'organization_id', value: params?.organizationId, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<PlanPage>(appendQueryString(appApiPath(`/github/plans`), query));
  }
}

export interface GithubIssuesListParams {
  tenantId?: string;
  organizationId?: string;
  page?: number;
  pageSize?: number;
  repositoryId?: string;
}

export interface GithubIssuesSyncParams {
  tenantId?: string;
  organizationId?: string;
  repositoryId?: string;
}

export class GithubIssuesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List GitHub issues. */
  async list(params?: GithubIssuesListParams): Promise<IssuePage> {
    const query = buildQueryString([
      { name: 'tenant_id', value: params?.tenantId, style: 'form', explode: true, allowReserved: false },
      { name: 'organization_id', value: params?.organizationId, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'repository_id', value: params?.repositoryId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<IssuePage>(appendQueryString(appApiPath(`/github/issues`), query));
  }

/** Sync issues from GitHub. */
  async sync(params?: GithubIssuesSyncParams): Promise<SyncResult> {
    const query = buildQueryString([
      { name: 'tenant_id', value: params?.tenantId, style: 'form', explode: true, allowReserved: false },
      { name: 'organization_id', value: params?.organizationId, style: 'form', explode: true, allowReserved: false },
      { name: 'repository_id', value: params?.repositoryId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.post<SyncResult>(appendQueryString(appApiPath(`/github/issues/sync`), query));
  }
}

export interface GithubRepositoriesListParams {
  tenantId?: string;
  organizationId?: string;
  page?: number;
  pageSize?: number;
}

export interface GithubRepositoriesSyncParams {
  tenantId?: string;
  organizationId?: string;
}

export class GithubRepositoriesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List tracked GitHub repositories. */
  async list(params?: GithubRepositoriesListParams): Promise<RepositoryPage> {
    const query = buildQueryString([
      { name: 'tenant_id', value: params?.tenantId, style: 'form', explode: true, allowReserved: false },
      { name: 'organization_id', value: params?.organizationId, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<RepositoryPage>(appendQueryString(appApiPath(`/github/repositories`), query));
  }

/** Sync repositories from GitHub. */
  async sync(params?: GithubRepositoriesSyncParams): Promise<SyncResult> {
    const query = buildQueryString([
      { name: 'tenant_id', value: params?.tenantId, style: 'form', explode: true, allowReserved: false },
      { name: 'organization_id', value: params?.organizationId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.post<SyncResult>(appendQueryString(appApiPath(`/github/repositories/sync`), query));
  }
}

export class GithubApi {
  private client: HttpClient;
  public readonly repositories: GithubRepositoriesApi;
  public readonly issues: GithubIssuesApi;
  public readonly plans: GithubPlansApi;
  public readonly integration: GithubIntegrationApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.repositories = new GithubRepositoriesApi(client);
    this.issues = new GithubIssuesApi(client);
    this.plans = new GithubPlansApi(client);
    this.integration = new GithubIntegrationApi(client);
  }

}

export function createGithubApi(client: HttpClient): GithubApi {
  return new GithubApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}


interface QueryParameterSpec {
  name: string;
  value: unknown;
  style: string;
  explode: boolean;
  allowReserved: boolean;
  contentType?: string;
}

function buildQueryString(parameters: QueryParameterSpec[]): string {
  const pairs: string[] = [];
  for (const parameter of parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

function appendSerializedParameter(pairs: string[], parameter: QueryParameterSpec): void {
  if (parameter.value === undefined || parameter.value === null) {
    return;
  }

  if (parameter.contentType) {
    pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(JSON.stringify(parameter.value), parameter.allowReserved)}`);
    return;
  }

  const style = parameter.style || 'form';
  if (style === 'deepObject') {
    appendDeepObjectParameter(pairs, parameter.name, parameter.value, parameter.allowReserved);
    return;
  }

  if (Array.isArray(parameter.value)) {
    appendArrayParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
    return;
  }

  if (typeof parameter.value === 'object') {
    appendObjectParameter(pairs, parameter.name, parameter.value as Record<string, unknown>, style, parameter.explode, parameter.allowReserved);
    return;
  }

  pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(serializePrimitive(parameter.value), parameter.allowReserved)}`);
}

function appendArrayParameter(
  pairs: string[],
  name: string,
  value: unknown[],
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const values = value
    .filter((item) => item !== undefined && item !== null)
    .map((item) => serializePrimitive(item));
  if (values.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const item of values) {
      pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(item, allowReserved)}`);
    }
    return;
  }

  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(values.join(','), allowReserved)}`);
}

function appendObjectParameter(
  pairs: string[],
  name: string,
  value: Record<string, unknown>,
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const [key, entryValue] of entries) {
      pairs.push(`${encodeQueryComponent(key)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
    }
    return;
  }

  const serialized = entries.flatMap(([key, entryValue]) => [key, serializePrimitive(entryValue)]).join(',');
  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serialized, allowReserved)}`);
}

function appendDeepObjectParameter(
  pairs: string[],
  name: string,
  value: unknown,
  allowReserved: boolean,
): void {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serializePrimitive(value), allowReserved)}`);
    return;
  }

  for (const [key, entryValue] of Object.entries(value as Record<string, unknown>)) {
    if (entryValue === undefined || entryValue === null) {
      continue;
    }
    pairs.push(`${encodeQueryComponent(`${name}[${key}]`)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
  }
}

function serializePrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}

function encodeQueryComponent(value: string): string {
  return encodeURIComponent(value);
}

function encodeQueryValue(value: string, allowReserved: boolean): string {
  const encoded = encodeURIComponent(value);
  if (!allowReserved) {
    return encoded;
  }
  return encoded.replace(/%3A/gi, ':')
    .replace(/%2F/gi, '/')
    .replace(/%3F/gi, '?')
    .replace(/%23/gi, '#')
    .replace(/%5B/gi, '[')
    .replace(/%5D/gi, ']')
    .replace(/%40/gi, '@')
    .replace(/%21/gi, '!')
    .replace(/%24/gi, '$')
    .replace(/%26/gi, '&')
    .replace(/%27/gi, "'")
    .replace(/%28/gi, '(')
    .replace(/%29/gi, ')')
    .replace(/%2A/gi, '*')
    .replace(/%2B/gi, '+')
    .replace(/%2C/gi, ',')
    .replace(/%3B/gi, ';')
    .replace(/%3D/gi, '=');
}

-- github provider account linking for tenant-scoped integration credentials

CREATE TABLE IF NOT EXISTS github_provider_account (
    id TEXT PRIMARY KEY NOT NULL,
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    provider TEXT NOT NULL DEFAULT 'github',
    external_account_id TEXT,
    access_token_cipher TEXT NOT NULL,
    scopes TEXT,
    status TEXT NOT NULL,
    last_synced_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (tenant_id, organization_id, provider)
);

CREATE INDEX IF NOT EXISTS idx_github_provider_account_scope
    ON github_provider_account (tenant_id, organization_id, provider, status);

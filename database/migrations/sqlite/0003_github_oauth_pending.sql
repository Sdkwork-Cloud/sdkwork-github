-- ephemeral OAuth state for GitHub integration linking

CREATE TABLE IF NOT EXISTS github_oauth_pending (
    state TEXT PRIMARY KEY NOT NULL,
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    expires_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_github_oauth_pending_expires
    ON github_oauth_pending (expires_at);

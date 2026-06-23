-- github integration baseline schema

CREATE TABLE IF NOT EXISTS github_repository (
    id TEXT PRIMARY KEY NOT NULL,
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    full_name TEXT NOT NULL,
    owner TEXT NOT NULL,
    description TEXT,
    default_branch TEXT,
    html_url TEXT,
    is_private INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS github_issue (
    id TEXT PRIMARY KEY NOT NULL,
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    repository_id TEXT NOT NULL,
    number INTEGER NOT NULL,
    title TEXT NOT NULL,
    state TEXT NOT NULL,
    html_url TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS github_plan (
    id TEXT PRIMARY KEY NOT NULL,
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    repository_id TEXT,
    title TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS github_plan_item (
    id TEXT PRIMARY KEY NOT NULL,
    plan_id TEXT NOT NULL,
    title TEXT NOT NULL,
    status TEXT NOT NULL,
    sort_order INTEGER NOT NULL,
    issue_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

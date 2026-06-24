-- Referential integrity indexes and SQLite relationship enforcement.

CREATE INDEX IF NOT EXISTS idx_github_issue_repository
    ON github_issue (repository_id);

CREATE INDEX IF NOT EXISTS idx_github_issue_scope
    ON github_issue (tenant_id, organization_id, repository_id);

CREATE INDEX IF NOT EXISTS idx_github_plan_repository
    ON github_plan (repository_id);

CREATE INDEX IF NOT EXISTS idx_github_plan_scope
    ON github_plan (tenant_id, organization_id, repository_id);

CREATE INDEX IF NOT EXISTS idx_github_plan_item_plan
    ON github_plan_item (plan_id);

CREATE INDEX IF NOT EXISTS idx_github_plan_item_issue
    ON github_plan_item (issue_id);

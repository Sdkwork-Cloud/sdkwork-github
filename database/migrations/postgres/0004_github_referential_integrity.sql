-- Referential integrity constraints for the GitHub integration schema.

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

ALTER TABLE github_issue
    DROP CONSTRAINT IF EXISTS fk_github_issue_repository;

ALTER TABLE github_issue
    ADD CONSTRAINT fk_github_issue_repository
    FOREIGN KEY (repository_id) REFERENCES github_repository (id) ON DELETE CASCADE;

ALTER TABLE github_plan
    DROP CONSTRAINT IF EXISTS fk_github_plan_repository;

ALTER TABLE github_plan
    ADD CONSTRAINT fk_github_plan_repository
    FOREIGN KEY (repository_id) REFERENCES github_repository (id) ON DELETE SET NULL;

ALTER TABLE github_plan_item
    DROP CONSTRAINT IF EXISTS fk_github_plan_item_plan;

ALTER TABLE github_plan_item
    ADD CONSTRAINT fk_github_plan_item_plan
    FOREIGN KEY (plan_id) REFERENCES github_plan (id) ON DELETE CASCADE;

ALTER TABLE github_plan_item
    DROP CONSTRAINT IF EXISTS fk_github_plan_item_issue;

ALTER TABLE github_plan_item
    ADD CONSTRAINT fk_github_plan_item_issue
    FOREIGN KEY (issue_id) REFERENCES github_issue (id) ON DELETE SET NULL;

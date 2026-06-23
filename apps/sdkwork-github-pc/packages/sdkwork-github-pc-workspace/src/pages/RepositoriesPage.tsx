import { useCallback, useEffect, useState } from 'react';
import { useGithubPcRuntime } from '@sdkwork/github-pc-core';
import { listRepositories, syncRepositories, getIntegrationStatus } from '../services/githubWorkspaceService';

export function RepositoriesPage() {
  const { githubSdk, session } = useGithubPcRuntime();
  const [items, setItems] = useState<Array<{ id: string; fullName: string; owner: string }>>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [syncing, setSyncing] = useState(false);
  const [syncMessage, setSyncMessage] = useState<string | null>(null);
  const [integrationLinked, setIntegrationLinked] = useState<boolean | null>(null);

  const loadRepositories = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const page = await listRepositories(githubSdk.client, session.getSnapshot().context);
      setItems(
        (page.items ?? []).map((item) => ({
          id: item.id,
          fullName: item.full_name,
          owner: item.owner,
        })),
      );
    } catch (reason: unknown) {
      setError(reason instanceof Error ? reason.message : String(reason));
    } finally {
      setLoading(false);
    }
  }, [githubSdk.client, session]);

  useEffect(() => {
    void loadRepositories();
    void getIntegrationStatus(githubSdk.client, session.getSnapshot().context)
      .then((status) => setIntegrationLinked(Boolean(status.linked)))
      .catch(() => setIntegrationLinked(false));
  }, [loadRepositories, githubSdk.client, session]);

  const handleSync = async () => {
    setSyncing(true);
    setSyncMessage(null);
    setError(null);
    try {
      const result = await syncRepositories(githubSdk.client, session.getSnapshot().context);
      setSyncMessage(`Synced ${result.synced_count ?? 0} repositories from ${result.provider ?? 'github'}.`);
      await loadRepositories();
    } catch (reason: unknown) {
      setError(reason instanceof Error ? reason.message : String(reason));
    } finally {
      setSyncing(false);
    }
  };

  if (loading) return <p>Loading repositories…</p>;

  return (
    <section>
      <header style={{ display: 'flex', gap: '12px', alignItems: 'center', marginBottom: '16px' }}>
        <h2 style={{ margin: 0 }}>Repositories</h2>
        {integrationLinked === false ? (
          <span>GitHub not linked for this organization.</span>
        ) : null}
        <button type="button" onClick={() => void handleSync()} disabled={syncing}>
          {syncing ? 'Syncing…' : 'Sync from GitHub'}
        </button>
      </header>
      {syncMessage ? <p>{syncMessage}</p> : null}
      {error ? <p role="alert">Failed to load repositories: {error}</p> : null}
      {!error && items.length === 0 ? <p>No repositories tracked yet. Sync from GitHub or seed demo data.</p> : null}
      {!error && items.length > 0 ? (
        <ul>
          {items.map((item) => (
            <li key={item.id}>
              <strong>{item.fullName}</strong> <span>({item.owner})</span>
            </li>
          ))}
        </ul>
      ) : null}
    </section>
  );
}

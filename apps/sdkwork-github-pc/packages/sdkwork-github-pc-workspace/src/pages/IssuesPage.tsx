import { useCallback, useEffect, useState } from 'react';
import { useGithubPcRuntime } from '@sdkwork/github-pc-core';
import { listIssues, syncIssues } from '../services/githubWorkspaceService';

export function IssuesPage() {
  const { githubSdk, session } = useGithubPcRuntime();
  const [items, setItems] = useState<Array<{ id: string; title: string; state: string }>>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [syncing, setSyncing] = useState(false);
  const [syncMessage, setSyncMessage] = useState<string | null>(null);

  const loadIssues = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const page = await listIssues(githubSdk.client, session.getSnapshot().context);
      setItems(
        (page.items ?? []).map((item) => ({
          id: item.id,
          title: item.title,
          state: item.state,
        })),
      );
    } catch (reason: unknown) {
      setError(reason instanceof Error ? reason.message : String(reason));
    } finally {
      setLoading(false);
    }
  }, [githubSdk.client, session]);

  useEffect(() => {
    void loadIssues();
  }, [loadIssues]);

  const handleSync = async () => {
    setSyncing(true);
    setSyncMessage(null);
    setError(null);
    try {
      const result = await syncIssues(githubSdk.client, session.getSnapshot().context);
      setSyncMessage(`Synced ${result.synced_count ?? 0} issues from ${result.provider ?? 'github'}.`);
      await loadIssues();
    } catch (reason: unknown) {
      setError(reason instanceof Error ? reason.message : String(reason));
    } finally {
      setSyncing(false);
    }
  };

  if (loading) return <p>Loading issues…</p>;

  return (
    <section>
      <header style={{ display: 'flex', gap: '12px', alignItems: 'center', marginBottom: '16px' }}>
        <h2 style={{ margin: 0 }}>Issues</h2>
        <button type="button" onClick={() => void handleSync()} disabled={syncing}>
          {syncing ? 'Syncing…' : 'Sync from GitHub'}
        </button>
      </header>
      {syncMessage ? <p>{syncMessage}</p> : null}
      {error ? <p role="alert">Failed to load issues: {error}</p> : null}
      {!error && items.length === 0 ? <p>No issues found. Sync from GitHub after linking integration.</p> : null}
      {!error && items.length > 0 ? (
        <ul>
          {items.map((item) => (
            <li key={item.id}>
              <strong>{item.title}</strong> <span>[{item.state}]</span>
            </li>
          ))}
        </ul>
      ) : null}
    </section>
  );
}

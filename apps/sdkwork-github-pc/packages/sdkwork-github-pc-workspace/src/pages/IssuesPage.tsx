import { useEffect, useState } from 'react';
import { useGithubPcRuntime } from '@sdkwork/github-pc-core';
import { listIssues } from '../services/githubWorkspaceService';

export function IssuesPage() {
  const { githubSdk, session } = useGithubPcRuntime();
  const [items, setItems] = useState<Array<{ id: string; title: string; state: string }>>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    void listIssues(githubSdk.client, session.getSnapshot().context)
      .then((page) => {
        if (cancelled) return;
        setItems(
          (page.items ?? []).map((item) => ({
            id: item.id,
            title: item.title,
            state: item.state,
          })),
        );
      })
      .catch((reason: unknown) => {
        if (cancelled) return;
        setError(reason instanceof Error ? reason.message : String(reason));
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [githubSdk.client, session]);

  if (loading) return <p>Loading issues…</p>;
  if (error) return <p role="alert">Failed to load issues: {error}</p>;
  if (items.length === 0) return <p>No issues found.</p>;

  return (
    <ul>
      {items.map((item) => (
        <li key={item.id}>
          <strong>{item.title}</strong> <span>[{item.state}]</span>
        </li>
      ))}
    </ul>
  );
}

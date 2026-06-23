import { useEffect, useState } from 'react';
import { useGithubPcRuntime } from '@sdkwork/github-pc-core';
import { listPlans } from '../services/githubWorkspaceService';

export function PlansPage() {
  const { githubSdk, session } = useGithubPcRuntime();
  const [items, setItems] = useState<Array<{ id: string; title: string; status: string }>>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    void listPlans(githubSdk.client, session.getSnapshot().context)
      .then((page) => {
        if (cancelled) return;
        setItems(
          (page.items ?? []).map((item) => ({
            id: item.id,
            title: item.title,
            status: item.status,
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

  if (loading) return <p>Loading plans…</p>;
  if (error) return <p role="alert">Failed to load plans: {error}</p>;
  if (items.length === 0) return <p>No plans found.</p>;

  return (
    <ul>
      {items.map((item) => (
        <li key={item.id}>
          <strong>{item.title}</strong> <span>[{item.status}]</span>
        </li>
      ))}
    </ul>
  );
}

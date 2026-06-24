import { useEffect, useState } from 'react';
import { useGithubPcRuntime } from '@sdkwork/github-pc-core';
import { listPlans } from '../services/githubWorkspaceService';

type PlanItemView = {
  id: string;
  title: string;
  status: string;
  sort_order: number;
  issue_id?: string;
};

type PlanView = {
  id: string;
  title: string;
  status: string;
  repository_id?: string;
  items: PlanItemView[];
};

export function PlansPage() {
  const { githubSdk, session } = useGithubPcRuntime();
  const [items, setItems] = useState<PlanView[]>([]);
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
            repository_id: item.repository_id,
            items: (item.items ?? []).map((planItem) => ({
              id: planItem.id,
              title: planItem.title,
              status: planItem.status,
              sort_order: planItem.sort_order,
              issue_id: planItem.issue_id,
            })),
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
  if (items.length === 0) return <p>No plans found. Initialize notable repositories or seed demo data.</p>;

  return (
    <section>
      <h2 style={{ marginTop: 0 }}>Plans</h2>
      <ul style={{ listStyle: 'none', padding: 0, display: 'grid', gap: '16px' }}>
        {items.map((plan) => (
          <li
            key={plan.id}
            style={{ border: '1px solid #ddd', borderRadius: '8px', padding: '12px 16px' }}
          >
            <header style={{ marginBottom: '8px' }}>
              <strong>{plan.title}</strong>{' '}
              <span>[{plan.status}]</span>
              {plan.repository_id ? (
                <span style={{ marginLeft: '8px', color: '#666' }}>repo: {plan.repository_id}</span>
              ) : null}
            </header>
            {plan.items.length === 0 ? (
              <p style={{ margin: 0, color: '#666' }}>No checklist items yet.</p>
            ) : (
              <ol style={{ margin: 0, paddingLeft: '20px' }}>
                {plan.items.map((planItem) => (
                  <li key={planItem.id}>
                    {planItem.title}{' '}
                    <span>[{planItem.status}]</span>
                    {planItem.issue_id ? (
                      <span style={{ marginLeft: '8px', color: '#0366d6' }}>
                        linked issue: {planItem.issue_id}
                      </span>
                    ) : null}
                  </li>
                ))}
              </ol>
            )}
          </li>
        ))}
      </ul>
    </section>
  );
}

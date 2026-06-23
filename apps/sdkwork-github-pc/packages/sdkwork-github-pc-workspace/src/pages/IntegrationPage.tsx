import { useCallback, useEffect, useMemo, useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import { useGithubPcRuntime } from '@sdkwork/github-pc-core';
import {
  beginOAuthIntegration,
  getIntegrationStatus,
  linkIntegration,
  unlinkIntegration,
} from '../services/githubWorkspaceService';

export function IntegrationPage() {
  const { githubSdk, session } = useGithubPcRuntime();
  const [searchParams] = useSearchParams();
  const [linked, setLinked] = useState<boolean | null>(null);
  const [statusMessage, setStatusMessage] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [submitting, setSubmitting] = useState(false);
  const [accessToken, setAccessToken] = useState('');

  const oauthResult = useMemo(() => {
    const linkedParam = searchParams.get('linked');
    if (linkedParam === '1') return 'GitHub account linked successfully.';
    if (linkedParam === '0') {
      const oauthError = searchParams.get('error');
      return oauthError ? `OAuth failed: ${oauthError}` : 'OAuth linking failed.';
    }
    return null;
  }, [searchParams]);

  const refreshStatus = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const status = await getIntegrationStatus(githubSdk.client, session.getSnapshot().context);
      setLinked(Boolean(status.linked));
    } catch (reason: unknown) {
      setError(reason instanceof Error ? reason.message : String(reason));
      setLinked(false);
    } finally {
      setLoading(false);
    }
  }, [githubSdk.client, session]);

  useEffect(() => {
    void refreshStatus();
  }, [refreshStatus]);

  const handleOAuth = async () => {
    setSubmitting(true);
    setError(null);
    setStatusMessage(null);
    try {
      const result = await beginOAuthIntegration(githubSdk.client, session.getSnapshot().context);
      if (!result.authorization_url) {
        throw new Error('authorization_url is missing from OAuth begin response');
      }
      window.location.assign(result.authorization_url);
    } catch (reason: unknown) {
      setError(reason instanceof Error ? reason.message : String(reason));
      setSubmitting(false);
    }
  };

  const handleLink = async () => {
    if (!accessToken.trim()) {
      setError('access token is required');
      return;
    }
    setSubmitting(true);
    setError(null);
    setStatusMessage(null);
    try {
      await linkIntegration(githubSdk.client, session.getSnapshot().context, accessToken.trim());
      setAccessToken('');
      setStatusMessage('GitHub PAT linked for this organization.');
      await refreshStatus();
    } catch (reason: unknown) {
      setError(reason instanceof Error ? reason.message : String(reason));
    } finally {
      setSubmitting(false);
    }
  };

  const handleUnlink = async () => {
    setSubmitting(true);
    setError(null);
    setStatusMessage(null);
    try {
      await unlinkIntegration(githubSdk.client, session.getSnapshot().context);
      setStatusMessage('GitHub integration unlinked.');
      await refreshStatus();
    } catch (reason: unknown) {
      setError(reason instanceof Error ? reason.message : String(reason));
    } finally {
      setSubmitting(false);
    }
  };

  if (loading) return <p>Loading integration status…</p>;

  return (
    <section>
      <h2>GitHub Integration</h2>
      {oauthResult ? <p>{oauthResult}</p> : null}
      {statusMessage ? <p>{statusMessage}</p> : null}
      {error ? <p role="alert">{error}</p> : null}
      <p>Status: {linked ? 'Linked' : 'Not linked'}</p>
      <div style={{ display: 'flex', flexDirection: 'column', gap: '12px', maxWidth: '480px' }}>
        <button type="button" onClick={() => void handleOAuth()} disabled={submitting}>
          {submitting ? 'Redirecting…' : 'Connect with GitHub OAuth'}
        </button>
        <label>
          Personal access token
          <input
            type="password"
            value={accessToken}
            onChange={(event) => setAccessToken(event.target.value)}
            placeholder="ghp_..."
            style={{ display: 'block', width: '100%', marginTop: '4px' }}
          />
        </label>
        <div style={{ display: 'flex', gap: '8px' }}>
          <button type="button" onClick={() => void handleLink()} disabled={submitting}>
            Link PAT
          </button>
          <button type="button" onClick={() => void handleUnlink()} disabled={submitting || !linked}>
            Unlink
          </button>
        </div>
      </div>
    </section>
  );
}

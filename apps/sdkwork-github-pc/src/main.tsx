import { StrictMode, useMemo } from 'react';
import { createRoot } from 'react-dom/client';
import { GithubRuntimeProvider } from '@sdkwork/github-pc-core';
import { createGithubPcRuntime } from './bootstrap/createGithubPcRuntime';
import { GithubAppRoutes } from './bootstrap/routes';

function Root() {
  const runtime = useMemo(() => createGithubPcRuntime(), []);
  return (
    <GithubRuntimeProvider runtime={runtime}>
      <GithubAppRoutes runtime={runtime} />
    </GithubRuntimeProvider>
  );
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Root />
  </StrictMode>,
);

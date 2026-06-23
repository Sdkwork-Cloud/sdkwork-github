import { APP_TITLE } from '@sdkwork/github-pc-core';

export function App() {
  return (
    <main style={{ fontFamily: 'system-ui, sans-serif', padding: '2rem' }}>
      <h1>{APP_TITLE}</h1>
      <p>Repositories, issues, and planning workspace.</p>
    </main>
  );
}

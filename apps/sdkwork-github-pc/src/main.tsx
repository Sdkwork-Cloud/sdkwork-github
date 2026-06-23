import React from 'react';
import { createRoot } from 'react-dom/client';
import { App } from '@sdkwork/github-pc-shell';

createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);

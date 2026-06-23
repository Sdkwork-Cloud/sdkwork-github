import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

const apiTarget = process.env.VITE_SDKWORK_GITHUB_APPLICATION_PUBLIC_HTTP_URL ?? 'http://127.0.0.1:4100';

export default defineConfig({
  plugins: [react()],
  server: {
    port: 5175,
    proxy: {
      '/app/v3/api': {
        target: apiTarget,
        changeOrigin: true,
      },
    },
  },
});

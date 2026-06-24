import { defineConfig, loadEnv } from 'vite';
import react from '@vitejs/plugin-react';

const defaultApiTarget = 'http://127.0.0.1:4100';

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, '.', '');
  const apiTarget =
    env.VITE_SDKWORK_GITHUB_APPLICATION_PUBLIC_HTTP_URL
    ?? process.env.VITE_SDKWORK_GITHUB_APPLICATION_PUBLIC_HTTP_URL
    ?? defaultApiTarget;

  return {
    define: {
      'process.env.SDKWORK_ACCESS_TOKEN': JSON.stringify(
        env.SDKWORK_ACCESS_TOKEN ?? process.env.SDKWORK_ACCESS_TOKEN ?? '',
      ),
    },
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
  };
});

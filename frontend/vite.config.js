import { defineConfig } from 'vite';

const apiTarget = process.env.API_PROXY_TARGET || process.env.VITE_API_URL || 'http://127.0.0.1:8000';

const apiProxy = {
  target: apiTarget,
  changeOrigin: true,
  rewrite: (path) => path.replace(/^\/api(?=\/|$)/, ''),
};

export default defineConfig({
  root: 'app',
  build: {
    outDir: '../dist',
    emptyOutDir: true,
  },
  server: {
    proxy: {
      '/api': {
        ...apiProxy,
      },
    },
  },
  preview: {
    proxy: {
      '/api': {
        ...apiProxy,
      },
    },
  },
});

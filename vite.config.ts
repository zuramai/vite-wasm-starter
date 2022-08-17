import { defineConfig } from 'vite'
import { ViteRsw } from 'vite-plugin-rsw';

export default defineConfig({
  base: 'vite-wasm-starter',
  plugins: [
    ViteRsw(),
  ]
})
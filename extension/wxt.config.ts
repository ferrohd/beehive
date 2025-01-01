import { defineConfig } from 'wxt';

// See https://wxt.dev/api/config.html
export default defineConfig({
  extensionApi: 'webextension-polyfill',
  srcDir: 'src',
  outDir: 'dist',
  modules: ['@wxt-dev/module-react'],
  manifest: {
    name: "CouponFinder",
    description: "Automatically finds and applies coupon codes while shopping",
    version: "1.0.0",
    manifest_version: 3,
    permissions: [
      "activeTab",
      "clipboardWrite",
      "scripting"
    ],
    host_permissions: [
      "*://*/*"
    ],
  }
});

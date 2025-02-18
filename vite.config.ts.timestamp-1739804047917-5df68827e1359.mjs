// vite.config.ts
import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "file:///home/ashenye/repo/gameboy-emulator/node_modules/.pnpm/vite@5.4.11_@types+node@20.17.11_sass@1.83.1/node_modules/vite/dist/node/index.js";
import vue from "file:///home/ashenye/repo/gameboy-emulator/node_modules/.pnpm/@vitejs+plugin-vue@5.2.1_vite@5.4.11_@types+node@20.17.11_sass@1.83.1__vue@3.5.13_typescript@5.4.5_/node_modules/@vitejs/plugin-vue/dist/index.mjs";
import VueDevTools from "file:///home/ashenye/repo/gameboy-emulator/node_modules/.pnpm/vite-plugin-vue-devtools@7.6.8_rollup@4.30.0_vite@5.4.11_@types+node@20.17.11_sass@1.83.1__vue@3.5.13_typescript@5.4.5_/node_modules/vite-plugin-vue-devtools/dist/vite.mjs";
import wasm from "file:///home/ashenye/repo/gameboy-emulator/node_modules/.pnpm/vite-plugin-wasm@3.4.1_vite@5.4.11_@types+node@20.17.11_sass@1.83.1_/node_modules/vite-plugin-wasm/exports/import.mjs";
import VueI18nPlugin from "file:///home/ashenye/repo/gameboy-emulator/node_modules/.pnpm/@intlify+unplugin-vue-i18n@4.0.0_rollup@4.30.0_vue-i18n@9.14.2_vue@3.5.13_typescript@5.4.5__/node_modules/@intlify/unplugin-vue-i18n/lib/vite.mjs";
import UnoCSS from "file:///home/ashenye/repo/gameboy-emulator/node_modules/.pnpm/unocss@0.59.4_postcss@8.4.49_rollup@4.30.0_vite@5.4.11_@types+node@20.17.11_sass@1.83.1_/node_modules/unocss/dist/vite.mjs";
var __vite_injected_original_import_meta_url = "file:///home/ashenye/repo/gameboy-emulator/vite.config.ts";
var vite_config_default = defineConfig({
  esbuild: {
    tsconfigRaw: {
      compilerOptions: {
        experimentalDecorators: true
      }
    }
  },
  plugins: [vue(), VueDevTools(), UnoCSS(), wasm(), VueI18nPlugin({})],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", __vite_injected_original_import_meta_url)),
      emulator: fileURLToPath(new URL("./emulator", __vite_injected_original_import_meta_url))
    }
  },
  css: {
    preprocessorOptions: {
      scss: {
        api: "modern"
      }
    }
  }
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvaG9tZS9hc2hlbnllL3JlcG8vZ2FtZWJveS1lbXVsYXRvclwiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9maWxlbmFtZSA9IFwiL2hvbWUvYXNoZW55ZS9yZXBvL2dhbWVib3ktZW11bGF0b3Ivdml0ZS5jb25maWcudHNcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfaW1wb3J0X21ldGFfdXJsID0gXCJmaWxlOi8vL2hvbWUvYXNoZW55ZS9yZXBvL2dhbWVib3ktZW11bGF0b3Ivdml0ZS5jb25maWcudHNcIjtpbXBvcnQgeyBmaWxlVVJMVG9QYXRoLCBVUkwgfSBmcm9tICdub2RlOnVybCdcblxuaW1wb3J0IHsgZGVmaW5lQ29uZmlnIH0gZnJvbSAndml0ZSdcbmltcG9ydCB2dWUgZnJvbSAnQHZpdGVqcy9wbHVnaW4tdnVlJ1xuaW1wb3J0IFZ1ZURldlRvb2xzIGZyb20gJ3ZpdGUtcGx1Z2luLXZ1ZS1kZXZ0b29scydcbmltcG9ydCB3YXNtIGZyb20gJ3ZpdGUtcGx1Z2luLXdhc20nXG5pbXBvcnQgVnVlSTE4blBsdWdpbiBmcm9tICdAaW50bGlmeS91bnBsdWdpbi12dWUtaTE4bi92aXRlJ1xuaW1wb3J0IFVub0NTUyBmcm9tICd1bm9jc3Mvdml0ZSdcblxuLy8gaHR0cHM6Ly92aXRlanMuZGV2L2NvbmZpZy9cbmV4cG9ydCBkZWZhdWx0IGRlZmluZUNvbmZpZyh7XG4gIGVzYnVpbGQ6IHtcbiAgICB0c2NvbmZpZ1Jhdzoge1xuICAgICAgY29tcGlsZXJPcHRpb25zOiB7XG4gICAgICAgIGV4cGVyaW1lbnRhbERlY29yYXRvcnM6IHRydWVcbiAgICAgIH1cbiAgICB9XG4gIH0sXG4gIHBsdWdpbnM6IFt2dWUoKSwgVnVlRGV2VG9vbHMoKSwgVW5vQ1NTKCksIHdhc20oKSwgVnVlSTE4blBsdWdpbih7fSldLFxuICByZXNvbHZlOiB7XG4gICAgYWxpYXM6IHtcbiAgICAgICdAJzogZmlsZVVSTFRvUGF0aChuZXcgVVJMKCcuL3NyYycsIGltcG9ydC5tZXRhLnVybCkpLFxuICAgICAgZW11bGF0b3I6IGZpbGVVUkxUb1BhdGgobmV3IFVSTCgnLi9lbXVsYXRvcicsIGltcG9ydC5tZXRhLnVybCkpXG4gICAgfVxuICB9LFxuICBjc3M6IHtcbiAgICBwcmVwcm9jZXNzb3JPcHRpb25zOiB7XG4gICAgICBzY3NzOiB7XG4gICAgICAgIGFwaTogJ21vZGVybidcbiAgICAgIH1cbiAgICB9XG4gIH1cbn0pXG4iXSwKICAibWFwcGluZ3MiOiAiO0FBQTJSLFNBQVMsZUFBZSxXQUFXO0FBRTlULFNBQVMsb0JBQW9CO0FBQzdCLE9BQU8sU0FBUztBQUNoQixPQUFPLGlCQUFpQjtBQUN4QixPQUFPLFVBQVU7QUFDakIsT0FBTyxtQkFBbUI7QUFDMUIsT0FBTyxZQUFZO0FBUDJKLElBQU0sMkNBQTJDO0FBVS9OLElBQU8sc0JBQVEsYUFBYTtBQUFBLEVBQzFCLFNBQVM7QUFBQSxJQUNQLGFBQWE7QUFBQSxNQUNYLGlCQUFpQjtBQUFBLFFBQ2Ysd0JBQXdCO0FBQUEsTUFDMUI7QUFBQSxJQUNGO0FBQUEsRUFDRjtBQUFBLEVBQ0EsU0FBUyxDQUFDLElBQUksR0FBRyxZQUFZLEdBQUcsT0FBTyxHQUFHLEtBQUssR0FBRyxjQUFjLENBQUMsQ0FBQyxDQUFDO0FBQUEsRUFDbkUsU0FBUztBQUFBLElBQ1AsT0FBTztBQUFBLE1BQ0wsS0FBSyxjQUFjLElBQUksSUFBSSxTQUFTLHdDQUFlLENBQUM7QUFBQSxNQUNwRCxVQUFVLGNBQWMsSUFBSSxJQUFJLGNBQWMsd0NBQWUsQ0FBQztBQUFBLElBQ2hFO0FBQUEsRUFDRjtBQUFBLEVBQ0EsS0FBSztBQUFBLElBQ0gscUJBQXFCO0FBQUEsTUFDbkIsTUFBTTtBQUFBLFFBQ0osS0FBSztBQUFBLE1BQ1A7QUFBQSxJQUNGO0FBQUEsRUFDRjtBQUNGLENBQUM7IiwKICAibmFtZXMiOiBbXQp9Cg==

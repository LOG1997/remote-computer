import path from "path"
import tailwindcss from "@tailwindcss/vite"
import viteReact from '@vitejs/plugin-react'
import { defineConfig } from "vite"
import { tanstackRouter } from '@tanstack/router-plugin/vite'
import { devtools } from '@tanstack/devtools-vite'
import { VitePWA } from 'vite-plugin-pwa'
// https://vite.dev/config/
export default defineConfig({
    base: '/',
    plugins: [
        tailwindcss(),
        tanstackRouter({
            target: 'react',
            autoCodeSplitting: true,
        }),
        viteReact(),
        devtools({
            removeDevtoolsOnBuild: true
        }),
        VitePWA({
            registerType: 'autoUpdate', // 自动检测更新+刷新
            manifest: {
                name: 'ShutdownRemote',
                short_name: 'ShutdownRemote',
                start_url: '/', // Hash根路由入口
                scope: '/',
                display: 'standalone', // 独立App窗口，无浏览器地址栏
                background_color: '#ffffff',
                theme_color: '#165DFF',
                icons: [
                    {
                        src: 'icon-192x192.png',
                        sizes: '192x192',
                        type: 'image/png'
                    },
                    {
                        src: 'icon-512x512.png',
                        sizes: '512x512',
                        type: 'image/png',
                    },
                    {
                        src: 'icon-192x192.png',
                        sizes: '192x192',
                        type: 'image/png',
                        purpose: 'any maskable',
                    },
                ]
            },
            workbox: {
                // Hash路由关键：所有#导航都走index.html兜底
                navigateFallback: 'index.html',
                // 匹配所有hash路由路径
                navigateFallbackAllowlist: [/./],
                globPatterns: ['**/*.{html,js,css,ico,png,svg}']
            },
            devOptions: {
                // 开发环境开启SW调试
                enabled: true,
                type: 'module'
            }
        }) as any

    ],
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "./src"),
        },
    },
    build: {
        outDir: '../../../server/web',
        assetsDir: 'assets',
    },
    preview: {
        port: 4575,
    },
})

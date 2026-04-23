import path from "path"
import tailwindcss from "@tailwindcss/vite"
import viteReact from '@vitejs/plugin-react'
import { defineConfig } from "vite"
import { tanstackRouter } from '@tanstack/router-plugin/vite'
import { devtools } from '@tanstack/devtools-vite'
// https://vite.dev/config/
export default defineConfig({
    base: './',
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
    ],
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "./src"),
        },
    },
    build: {
        outDir: 'dist',
        assetsDir: 'assets',
    },
    preview: {
        port: 4575,
    },
})

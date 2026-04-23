import js from '@eslint/js'
import globals from 'globals'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import tseslint from 'typescript-eslint'
import { defineConfig, globalIgnores } from 'eslint/config'

export default defineConfig([
    globalIgnores(['dist']),
    {
        files: ['**/*.{ts,tsx}'],
        extends: [
            js.configs.recommended,
            tseslint.configs.recommended,
            reactHooks.configs.flat.recommended,
            reactRefresh.configs.vite,
        ],
        languageOptions: {
            ecmaVersion: 2020,
            globals: globals.browser,
            parser: tseslint.parser,
            parserOptions: {
                // 关键修复：显式指定 tsconfig 根目录
                // 假设当前 eslint 配置文件位于 /home/log1997/r/shutdown/client/apps/web 下
                tsconfigRootDir: resolve(__dirname),
                project: ['./tsconfig.json'], // 确保指向正确的 tsconfig 文件
            },
        },
    },
])

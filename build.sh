#!/bin/bash
echo "=============================================="
echo "  开始一键打包：Rust后端 + 前端网页"
echo "=============================================="

rm ./dist -rf

# 1. 打包 Rust
echo "[1/4] 打包 Rust 后端..."
cd server
cargo build --release
cd ..

# 2. 打包前端
echo "[2/4] 打包前端..."
cd client
pnpm build
cd ..

# 3. 重建 dist
echo "[3/4] 重建 dist 目录..."
rm -rf dist
mkdir -p dist/web

# 4. 复制文件
echo "[4/4] 复制文件..."
cp server/target/release/shutdown-auto dist/
cp server/cert.pem dist/
cp server/key.pem dist/
cp server/config.toml dist/
cp -r client/apps/web/dist/* dist/web/

# 5. 压缩包，把dist内的文件夹和文件压缩成shutdown-remote.zip，放在dist目录中，打包最小化
echo "打包压缩..."
cd dist
zip -r shutdown-remote.zip *


echo ""
echo "=============================================="
echo "  ✅ 打包完成！产物在 dist 文件夹"
echo "=============================================="
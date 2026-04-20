# build.ps1
# 确保以管理员权限或允许执行脚本的策略运行
# Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned (如果首次运行报错)

Write-Host "=============================================="
Write-Host "  开始一键打包：Rust后端 + 前端网页"
Write-Host "=============================================="

# 清理旧的 dist 目录 (如果存在)
if (Test-Path "./dist") {
    Remove-Item -Path "./dist" -Recurse -Force
}

# 1. 打包 Rust
Write-Host "[1/4] 打包 Rust 后端..."
Push-Location server
try {
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        throw "Cargo build failed with exit code $LASTEXITCODE"
    }
}
finally {
    Pop-Location
}

# 2. 打包前端
Write-Host "[2/4] 打包前端..."
Push-Location client
try {
    pnpm build
    if ($LASTEXITCODE -ne 0) {
        throw "Pnpm build failed with exit code $LASTEXITCODE"
    }
}
finally {
    Pop-Location
}

# 3. 重建 dist 目录结构
Write-Host "[3/4] 重建 dist 目录..."
New-Item -ItemType Directory -Force -Path "./dist/web" | Out-Null

# 4. 复制文件
Write-Host "[4/4] 复制文件..."

# 复制 Rust 可执行文件
# 注意：Windows 下 Rust release 产物通常是 .exe
$exeSource = "server/target/release/shutdown-auto.exe"
if (Test-Path $exeSource) {
    Copy-Item -Path $exeSource -Destination "./dist/"
} else {
    # 尝试不带 .exe 的情况（虽然少见，但以防万一）
    $exeSourceFallback = "server/target/release/shutdown-auto"
    if (Test-Path $exeSourceFallback) {
        Copy-Item -Path $exeSourceFallback -Destination "./dist/"
    } else {
        Write-Error "未找到 Rust 编译产物: shutdown-auto.exe"
        exit 1
    }
}

# 复制证书文件
Copy-Item -Path "server/cert.pem" -Destination "./dist/"
Copy-Item -Path "server/key.pem" -Destination "./dist/"

# 复制前端构建产物 (递归复制目录内容)
Copy-Item -Path "client/apps/web/dist/*" -Destination "./dist/web/" -Recurse

Write-Host ""
Write-Host "=============================================="
Write-Host "  ✅ 打包完成！产物在 dist 文件夹"
Write-Host "=============================================="
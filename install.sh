#!/bin/bash

# 定义变量
SOURCE_DIR="./"
# 建议安装在系统共享目录，这样所有用户都能访问，且符合 Linux 规范
# 如果必须安装在当前用户主目录，请改为: TARGET_DIR="$HOME/remote-computer"
TARGET_DIR="/usr/local/share/remote-computer"
SERVICE_NAME="remote-computer"
SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"

# 检查源目录是否存在
if [ ! -d "$SOURCE_DIR" ]; then
    echo "错误: 找不到源目录 '$SOURCE_DIR'。请确保在包含 dist 文件夹的目录下运行此脚本。"
    exit 1
fi

# 检查是否以 root 权限运行，如果不是，尝试重新以 sudo 运行
if [ "$EUID" -ne 0 ]; then
    echo "检测到非 root 权限，正在请求 sudo 权限..."
    sudo "$0" "$@"
    exit $?
fi

echo "正在安装 ${SERVICE_NAME}..."

# 首先检查服务是否已存在
NEED_REINSTALL=false
if systemctl list-unit-files | grep -q "${SERVICE_NAME}.service"; then
    echo "⚠️  警告: 系统服务 '${SERVICE_NAME}' 已存在。"
    read -p "是否重新安装？(y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        NEED_REINSTALL=true
        echo "将执行重新安装..."
        
        # 停止并禁用旧服务
        echo "正在停止旧服务..."
        systemctl stop "$SERVICE_NAME" 2>/dev/null || true
        systemctl disable "$SERVICE_NAME" 2>/dev/null || true
        
        # 删除旧服务文件
        echo "正在删除旧服务文件..."
        rm -f "$SERVICE_FILE"
        
        # 重载 systemd
        systemctl daemon-reload
        
        # 清空目标目录
        if [ -d "$TARGET_DIR" ]; then
            echo "正在清空目标目录..."
            rm -rf "$TARGET_DIR"
        fi
        
        echo "旧版本已清理完毕。"
    else
        echo "❌ 安装已取消。"
        exit 0
    fi
else
    # 服务不存在，检查目标目录是否有残留文件
    if [ -d "$TARGET_DIR" ] && [ "$(ls -A "$TARGET_DIR" 2>/dev/null)" ]; then
        echo "⚠️  警告: 目标目录 '$TARGET_DIR' 已存在且包含文件（但服务未注册）。"
        read -p "是否清空并重新安装？(y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            NEED_REINSTALL=true
            echo "正在清空目标目录..."
            rm -rf "$TARGET_DIR"
        else
            echo "❌ 安装已取消。"
            exit 0
        fi
    fi
fi

# 1. 复制文件到目标目录
echo "正在复制文件到 ${TARGET_DIR} ..."

# 创建目标目录并复制
mkdir -p "$TARGET_DIR"
cp -r "${SOURCE_DIR}/." "$TARGET_DIR/"

if [ $? -ne 0 ]; then
    echo "错误: 文件复制失败。"
    exit 1
fi
echo "文件复制完成。"

# 2. 查找可执行文件
# 假设 dist 目录下有一个主要的可执行文件，例如 shutdown-auto 或 server
# 请根据实际文件名修改下面的 EXECUTABLE 变量
EXECUTABLE=""

# 尝试查找常见的可执行文件
if [ -f "$TARGET_DIR/shutdown-auto" ]; then
    EXECUTABLE="$TARGET_DIR/shutdown-auto"
elif [ -f "$TARGET_DIR/server" ]; then
    EXECUTABLE="$TARGET_DIR/server"
elif [ -f "$TARGET_DIR/main" ]; then
    EXECUTABLE="$TARGET_DIR/main"
else
    # 如果没有找到明确的二进制文件，假设是一个 Node.js 应用或其他需要解释器的应用
    echo "警告: 未在 dist 目录中找到默认的可执行文件 (shutdown-auto, server, main)。"
    echo "请手动编辑脚本中的 EXECUTABLE 变量，指定正确的启动命令。"
    # 为了演示，我们假设有一个名为 shutdown-auto 的文件，如果不存在，服务可能会启动失败
    EXECUTABLE="$TARGET_DIR/shutdown-auto" 
fi

# 确保可执行文件有执行权限
chmod +x "$EXECUTABLE" 2>/dev/null || true

# 3. 创建 Systemd 服务文件
echo "正在创建 Systemd 服务文件..."

cat > "$SERVICE_FILE" <<EOF
[Unit]
Description=Shutdown Remote Service
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=${TARGET_DIR}
ExecStart=${EXECUTABLE}
Restart=on-failure
RestartSec=5s
StandardOutput=journal
StandardError=journal
Environment=RUST_BACKTRACE=1

[Install]
WantedBy=multi-user.target
EOF

if [ $? -ne 0 ]; then
    echo "错误: 创建服务文件失败。"
    exit 1
fi

# 4. 重载 systemd 配置，启用并启动服务
echo "正在重载 systemd 配置..."
systemctl daemon-reload

echo "正在启用开机自启..."
systemctl enable "$SERVICE_NAME"

echo "正在启动服务..."
systemctl start "$SERVICE_NAME"

# 检查服务状态
sleep 10
if systemctl is-active --quiet "$SERVICE_NAME"; then
    echo "✅ 安装成功！${SERVICE_NAME} 已启动并设置为开机自启。"
    echo "   服务状态: $(systemctl status "$SERVICE_NAME" --no-pager -l | head -n 5)"
    echo "   日志查看: journalctl -u ${SERVICE_NAME} -f"
else
    echo "❌ 服务启动失败。请检查日志："
    echo "journalctl -u ${SERVICE_NAME} -e"
    exit 1
fi
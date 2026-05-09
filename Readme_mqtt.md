# 接入homeassistant

## 1. 安装mqtt服务

安装并启动一个mqtt服务

请注意填写`./config.toml`中的`mqtt`字段:

```toml
[mqtt]
enable = true // 把否启用mqtt功能
host = "127.0.0.1" // mqtt服务地址
port = 1883 // mqtt服务端口
client_id = "pc_pc01" // mqtt客户端id，建议使用唯一的id，避免与其他设备冲突
device_name = "我的电脑" // 设备名称
device_model = "windows11" // 设备型号
username = "ha_log1997" // mqtt用户名，如果mqtt服务需要认证，请填写此项
password = "123456" // mqtt密码，如果mqtt服务需要认证，请填写此项
tls = false // 是否使用tls，不使用（没做此项）
interval = 10 // 检测间隔
expire_time = 20 // 过期时间，单位秒，如果超过这个时间没有收到设备的状态更新，homeassistant会认为设备离线
// 启动应用列表，key为应用的可执行文件名，value为在homeassistant中显示的名称
launch_app = { "wechat" = "微信", "steam" = "Steam", "tailscale" = "Tailscale", "google-chrome" = "chrome浏览器", "code" = "Vs Code", "zed" = "Zed", "clash-verge" = "Clash Verge" }

```

如果mqtt服务设置了用户名和密码，请将`username`和`password`字段填写上。

如果你的mqtt服务权限设置中设置了允许账号只能订阅哪些主题，请将`"homeassistant/#", "ha_state/#", "ha_control/#"`三个主题添加进去。这三个主题分别是homeassistant的发现主题、本应用发送状态更新主题、本应用发送控制命令主题。

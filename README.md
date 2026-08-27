# android-scan-simulator

一款 Windows 桌面 GUI 应用程序，模拟 Android PDA 的扫码输入，主要用于模拟器调试仿真扫码过程。

## 功能特性

- **ADB 设备选择器** - 列出已连接的 Android 设备，支持刷新
- **广播发送器** - 通过 `am broadcast` intent 发送条码数据
- **条码图像解码** - 从图像文件解码条码（支持 QR 码及所有线性格式）
- **剪贴板图像解码** - 从剪贴板粘贴图像（Ctrl+V）进行解码
- **配置文件** - 用于不同广播配置的命名预设
- **持久化配置** - 将设置保存到二进制文件旁的 `config.toml`

## 环境要求

- 运行时需要 `adb` 在系统 PATH 中

## 构建与运行

- Rust 1.86+（edition 2024）

```bash
cargo build          # 调试构建
cargo run            # 运行应用
cargo build --release  # 发布构建
```

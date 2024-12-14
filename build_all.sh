#!/bin/bash

# 编译 Linux x86_64 静态二进制文件
echo "Building for Linux x86_64..."
cross build --release --target x86_64-unknown-linux-musl

# 编译 Linux ARM (armv7) 静态二进制文件
echo "Building for Linux ARM (armv7)..."
cross build --release --target armv7-unknown-linux-musleabihf

# 编译 Linux ARM64 (aarch64) 静态二进制文件
echo "Building for Linux ARM64 (aarch64)..."
cross build --release --target aarch64-unknown-linux-musl

# 编译 macOS x86_64 静态二进制文件
echo "Building for macOS x86_64..."
cross build --release --target x86_64-apple-darwin

# 编译 macOS ARM64 (Apple Silicon) 静态二进制文件
echo "Building for macOS ARM64 (Apple Silicon)..."
cross build --release --target aarch64-apple-darwin

# 编译 Windows x86 静态二进制文件
echo "Building for Windows x86..."
cross build --release --target i686-pc-windows-gnu

# 打包所有二进制文件
echo "Packaging binaries..."
mkdir -p artifacts
cp target/x86_64-unknown-linux-musl/release/your_binary artifacts/linux_x86
cp target/armv7-unknown-linux-musleabihf/release/your_binary artifacts/linux_arm
cp target/aarch64-unknown-linux-musl/release/your_binary artifacts/linux_arm64
cp target/x86_64-apple-darwin/release/your_binary artifacts/macos_x86
cp target/aarch64-apple-darwin/release/your_binary artifacts/macos_arm64
cp target/i686-pc-windows-gnu/release/your_binary.exe artifacts/windows_x86.exe

# 压缩打包
tar -czvf artifacts.tar.gz artifacts/

echo "Build and packaging complete!"
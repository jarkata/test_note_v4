#!/bin/bash

# 构建脚本: 编译 Rust 代码为 iOS 库

set -e

RUST_TARGET_DIR="target"
IOS_LIB_DIR="ios/NoteApp/RustLib"

echo "🔨 开始构建 Rust iOS 库..."

# 创建输出目录
mkdir -p "$IOS_LIB_DIR"

# 添加 iOS 编译目标
rustup target add aarch64-apple-ios x86_64-apple-ios

# 为 iOS 设备编译 (ARM64)
echo "📱 编译 iOS 设备版本 (aarch64-apple-ios)..."
cargo build --release --target aarch64-apple-ios

# 为 iOS 模拟器编译 (x86_64)
echo "📱 编译 iOS 模拟器版本 (x86_64-apple-ios)..."
cargo build --release --target x86_64-apple-ios

# 创建 Universal Binary (模拟器 + 设备)
echo "📦 创建 Universal Binary..."
mkdir -p "$IOS_LIB_DIR"

lipo -create \
    "$RUST_TARGET_DIR/aarch64-apple-ios/release/libnote_logic.a" \
    "$RUST_TARGET_DIR/x86_64-apple-ios/release/libnote_logic.a" \
    -output "$IOS_LIB_DIR/libnote_logic.a"

# 复制头文件
echo "📋 复制头文件..."
cp src/lib.rs "$IOS_LIB_DIR/note_logic.h" 2>/dev/null || echo "注意: 手动创建 Bridging Header"

echo "✅ 构建完成!"
echo "库文件位置: $IOS_LIB_DIR/libnote_logic.a"

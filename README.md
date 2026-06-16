# Rust + SwiftUI iOS 笔记应用

一个使用 **Rust** 实现业务逻辑，**SwiftUI** 开发 UI 的原生 iOS 应用。

## 项目架构

```
note_app/
├── src/
│   └── lib.rs                 # Rust 业务逻辑层 (FFI 接口)
├── Cargo.toml                 # Rust 依赖配置
├── ios/
│   └── NoteApp/
│       ├── App.swift          # SwiftUI 主入口
│       ├── ContentView.swift  # 主界面 (笔记列表)
│       ├── RustBridge.swift   # Swift-Rust 桥接层
│       └── NoteApp.xcodeproj  # Xcode 工程
└── build.sh                   # 编译脚本
```

## 技术栈

### 后端 - Rust
- **Serde**: JSON 序列化/反序列化
- **FFI (Foreign Function Interface)**: 与 Swift 的互操作
- 业务逻辑: 笔记的增删改查、搜索等

### 前端 - iOS/SwiftUI
- **SwiftUI**: 声明式 UI 框架
- **iOS 15+**: 最低部署目标
- **Swift**: 与 Rust 通过 FFI 通信

## 功能特性

✅ **创建笔记** - 添加标题和内容
✅ **查看笔记** - 浏览笔记详情
✅ **编辑笔记** - 修改现有笔记
✅ **删除笔记** - 移除不需要的笔记
✅ **搜索功能** - 按关键词搜索笔记
✅ **时间戳** - 记录创建和更新时间

## 快速开始

### 前提条件

- **Rust**: 1.70+ (安装: [rustup.rs](https://rustup.rs))
- **Xcode**: 14.0+ (含 Command Line Tools)
- **macOS**: 12.0+

### 1. 构建 Rust 库

```bash
# 添加 iOS 编译目标
rustup target add aarch64-apple-ios x86_64-apple-ios

# 执行构建脚本
chmod +x build.sh
./build.sh
```

这会生成:
- `ios/NoteApp/RustLib/libnote_logic.a` - Universal Binary (ARM64 + x86_64)

### 2. 在 Xcode 中打开项目

```bash
open ios/NoteApp/NoteApp.xcodeproj
```

### 3. 配置 Xcode 项目

#### 3.1 添加 Rust 库到 Xcode

1. 在 Xcode 中选择 Target `NoteApp`
2. 进入 **Build Phases** → **Link Binary With Libraries**
3. 点击 `+` 添加 `libnote_logic.a`

#### 3.2 配置 Build Settings

1. 搜索 `Library Search Paths`
2. 添加: `$(SRCROOT)/RustLib`

#### 3.3 创建 Bridging Header (如需)

如果 Swift 无法识别 Rust 函数，创建 `NoteApp-Bridging-Header.h`:

```objective-c
//
//  NoteApp-Bridging-Header.h
//  NoteApp
//

#ifndef NoteApp_Bridging_Header_h
#define NoteApp_Bridging_Header_h

// Rust FFI 声明将在这里暴露给 Swift

#endif /* NoteApp_Bridging_Header_h */
```

### 4. 编译并运行

1. 选择 Target: **NoteApp** > **iPhone Simulator** 或 **真机**
2. 按 `Cmd + R` 运行应用
3. 或从 Xcode 菜单: **Product** > **Run**

## API 说明

### Rust FFI 接口

#### 初始化
```rust
pub extern "C" fn note_manager_init()
```
初始化笔记管理器。在应用启动时调用。

#### 创建笔记
```rust
pub extern "C" fn create_note(
    title: *const c_char,
    content: *const c_char,
) -> *const c_char
```
返回 JSON 格式的 Note 对象。

#### 获取所有笔记
```rust
pub extern "C" fn get_all_notes() -> *const c_char
```
返回 JSON 数组。

#### 更新笔记
```rust
pub extern "C" fn update_note(
    id: *const c_char,
    title: *const c_char,
    content: *const c_char,
) -> *const c_char
```

#### 删除笔记
```rust
pub extern "C" fn delete_note(id: *const c_char) -> bool
```

#### 搜索笔记
```rust
pub extern "C" fn search_notes(keyword: *const c_char) -> *const c_char
```

### Swift 封装

`RustBridge.swift` 提供了便利的 Swift 接口:

```swift
// 创建笔记
RustBridge.shared.createNote(title: "My Note", content: "Content...")

// 获取所有笔记
let notes = RustBridge.shared.getAllNotes()

// 搜索
let results = RustBridge.shared.searchNotes(keyword: "关键词")

// 删除
RustBridge.shared.deleteNote(id: "note-id")
```

## 项目结构详解

### Rust 层 (`src/lib.rs`)

```rust
pub struct Note { ... }        // 笔记数据结构
pub struct NoteManager { ... } // 笔记管理核心逻辑
// FFI 导出函数
pub extern "C" fn create_note(...) { ... }
```

### Swift 层

```
RustBridge.swift    // FFI 声明 + Swift 包装
  ├─ C 函数声明
  ├─ Note 数据模型
  └─ RustBridge 管理器

ContentView.swift   // 主笔记列表界面
NoteDetailView.swift// 笔记详情 + 编辑
CreateNoteSheet.swift// 新建笔记表单
SearchBar.swift     // 搜索栏组件
```

## 构建优化

### 编译大小优化

在 `Cargo.toml` 中已配置:

```toml
[profile.release]
opt-level = 3        # 最高优化
lto = true           # Link Time Optimization
codegen-units = 1    # 单代码生成单元
strip = true         # 移除符号表
```

### iOS 应用包大小

- Rust 库: ~1-2 MB
- Swift 代码: ~0.5 MB
- 总体 App 大小: 取决于资源和其他依赖

## 常见问题

### Q: Xcode 找不到 Rust 库?

**A**: 检查:
1. `RustLib/libnote_logic.a` 是否存在
2. Build Settings 中的 Library Search Paths
3. Link Binary With Libraries 中是否添加了库

### Q: 编译 Rust 时出错?

**A**: 
```bash
# 更新 Rust
rustup update

# 清理构建
cargo clean

# 重新构建
./build.sh
```

### Q: 如何在真机上测试?

**A**:
1. 配置 Apple Developer Account
2. 选择 Team
3. 连接 iPhone
4. 选择设备并运行

## 部署

### App Store 发布流程

1. **配置签名证书**
   - 在 Xcode 中设置 Team ID
   - 配置 Bundle Identifier

2. **编译发行版**
   ```bash
   # Release 构建
   ./build.sh
   ```

3. **存档应用**
   - Xcode: Product > Archive

4. **上传到 App Store**
   - 使用 Transporter 或 Xcode 上传

## 扩展建议

- 🔐 添加数据持久化 (SQLite/CoreData)
- 🎨 支持暗黑模式
- 📝 富文本编辑
- 🏷️ 标签分类
- 📱 iCloud 同步
- 🔔 提醒功能

## 许可证

MIT License

## 作者

GitHub: [@jarkata](https://github.com/jarkata)

---

**开始构建您的 iOS 笔记应用吧！** 🚀

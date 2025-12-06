# 开发指南

本文档提供 NeuraDock 开发的完整命令参考和最佳实践。

## 目录

- [快速开始](#快速开始)
- [完整命令参考](#完整命令参考)
- [常用工作流](#常用工作流)
- [故障排除](#故障排除)

---

## 快速开始

### 首次设置

```bash
# 1. 克隆仓库
git clone https://github.com/i-rtfsc/NeuraDock.git
cd NeuraDock

# 2. 安装依赖
make setup

# 3. 启动开发服务器
make dev
```

### 日常开发

```bash
# 启动开发服务器
make dev

# 快速启动（跳过依赖检查）
make dev-fast

# 查看日志
make logs

# 重启服务器
make kill dev
```

---

## 完整命令参考

### 📦 安装和依赖

| 命令 | 说明 | 使用场景 |
|-----|------|---------|
| `make setup` | 首次安装所有依赖 | 首次克隆仓库后 |
| `make install` | 同 setup | 与 setup 相同 |
| `make check-deps` | 检查依赖是否已安装 | 验证环境 |
| `make update-deps` | 更新所有依赖 | 定期维护 |
| `make outdated` | 检查过时的依赖 | 查看可更新的包 |
| `make install-rust-tools` | 安装 Rust 开发工具 | 设置开发环境 |

**示例：**
```bash
# 首次安装
make setup

# 定期更新依赖
make update-deps

# 检查哪些包过时了
make outdated
```

### 🚀 开发模式

| 命令 | 说明 | 日志级别 |
|-----|------|---------|
| `make dev` | 启动开发模式 | info（标准） |
| `make dev-debug` | 启动开发模式 | debug（详细） |
| `make dev-trace` | 启动开发模式 | trace（追踪） |
| `make dev-warn` | 启动开发模式 | warn（仅警告） |
| `make dev-fast` | 快速启动 | info（跳过检查） |
| `make dev-first` | 首次运行 | info（自动安装） |
| `make kill` | 杀掉所有进程 | - |

**示例：**
```bash
# 标准开发
make dev

# 需要详细日志时
make dev-debug

# 性能分析时
make dev-trace

# 快速启动（适合频繁重启）
make dev-fast

# 强制重启
make kill dev
```

### 📦 构建命令

| 命令 | 说明 | 输出 |
|-----|------|------|
| `make build` | 构建 Release 版本 | 二进制文件 |
| `make build-release` | 构建并打包 | 安装包 (.dmg/.msi/.AppImage) |
| `make build-release-fast` | 快速构建 | 二进制文件（不打包） |
| `make build-frontend` | 仅构建前端 | dist/ 目录 |
| `make build-backend` | 仅构建后端 | target/release/ |
| `make run-release` | 运行 Release 版本 | - |
| `make rebuild` | 清理后重新构建 | 二进制文件 |
| `make bindings` | 生成 TypeScript 绑定 | src/lib/tauri.ts |

**示例：**
```bash
# 开发构建（快速）
make build

# 生产构建（完整打包）
make build-release

# 测试 Release 版本
make build-release-fast
make run-release

# 仅更新前端
make build-frontend
```

**构建产物位置：**
- macOS: `apps/desktop/src-tauri/target/release/bundle/dmg/`
- Windows: `apps/desktop/src-tauri/target/release/bundle/msi/`
- Linux: `apps/desktop/src-tauri/target/release/bundle/appimage/`

### 🧪 测试命令

| 命令 | 说明 | 输出 |
|-----|------|------|
| `make test` | 运行所有测试 | 测试结果 |
| `make test-backend` | 运行后端测试 | 测试结果 |
| `make test-coverage` | 生成覆盖率报告 | HTML/JSON/LCOV |
| `make coverage-report` | 打开覆盖率报告 | 在浏览器中打开 |

**示例：**
```bash
# 快速测试
make test-backend

# 生成并查看覆盖率
make test-coverage
make coverage-report
```

**覆盖率报告位置：**
- HTML: `apps/desktop/src-tauri/coverage/tarpaulin-report.html`
- JSON: `apps/desktop/src-tauri/coverage/tarpaulin-report.json`
- LCOV: `apps/desktop/src-tauri/coverage/lcov.info`

### 🧹 清理命令

| 命令 | 说明 | 删除内容 |
|-----|------|---------|
| `make clean` | 清理构建产物 | dist/ + target/ |
| `make clean-frontend` | 清理前端 | dist/ + .vite/ |
| `make clean-backend` | 清理后端 | target/ + coverage/ |
| `make clean-all` | 深度清理 | 以上 + node_modules/ + 日志 + 数据库 |

**示例：**
```bash
# 日常清理
make clean

# 完全重置（重新安装依赖）
make clean-all
make setup
```

**清理内容详情：**
- `clean`: 删除构建产物（~13GB）
- `clean-all`: 删除所有内容，包括：
  - `node_modules/` (~350MB)
  - `target/` (~13GB)
  - 日志文件
  - 数据库文件

### ✅ 代码质量

| 命令 | 说明 | 工具 |
|-----|------|------|
| `make check` | 检查代码格式 | rustfmt + clippy |
| `make fix` | 自动修复格式 | rustfmt |

**示例：**
```bash
# 提交前检查
make check

# 自动修复格式问题
make fix

# 完整检查流程
make fix
make check
make test-backend
```

### 🔧 工具和信息

| 命令 | 说明 | 用途 |
|-----|------|------|
| `make env-check` | 检查开发环境 | 验证工具安装 |
| `make version` | 显示版本信息 | 查看版本号 |
| `make status` | 查看项目状态 | 查看依赖和端口 |
| `make migrate` | 运行数据库迁移 | 更新数据库结构 |
| `make logs` | 查看今天的日志 | 调试问题 |
| `make fix-permissions` | 修复文件权限 | 解决权限问题 |
| `make help` | 显示帮助信息 | 查看所有命令 |

**示例：**
```bash
# 验证开发环境
make env-check

# 查看版本
make version

# 查看项目状态
make status

# 查看日志
make logs

# 获取帮助
make help
```

---

## 常用工作流

### 1. 日常开发流程

```bash
# 1. 启动开发
make dev

# 2. 编写代码...

# 3. 测试
make test-backend

# 4. 检查格式
make check

# 5. 自动修复
make fix

# 6. 提交代码
git add .
git commit -m "feat: ..."
```

### 2. 发布流程

```bash
# 1. 更新版本号
# 编辑 apps/desktop/src-tauri/Cargo.toml

# 2. 清理旧构建
make clean-all

# 3. 重新安装依赖
make setup

# 4. 运行测试
make test-backend

# 5. 构建 Release 版本
make build-release

# 6. 测试 Release 版本
make run-release

# 7. 查看构建产物
ls -lh apps/desktop/src-tauri/target/release/bundle/*/
```

### 3. 调试流程

```bash
# 1. 使用 debug 日志启动
make dev-debug

# 2. 查看实时日志
make logs

# 3. 如果需要性能分析
make kill
make dev-trace

# 4. 运行测试定位问题
make test-backend

# 5. 生成覆盖率报告
make test-coverage
make coverage-report
```

### 4. 依赖更新流程

```bash
# 1. 检查过时的依赖
make outdated

# 2. 更新依赖
make update-deps

# 3. 测试
make test-backend

# 4. 如有问题，回滚
git checkout -- apps/desktop/package.json
git checkout -- apps/desktop/src-tauri/Cargo.toml
make setup
```

### 5. 完全重置流程

```bash
# 1. 深度清理
make clean-all

# 2. 重新安装依赖
make setup

# 3. 验证环境
make env-check

# 4. 启动开发
make dev
```

---

## 故障排除

### 问题：依赖安装失败

```bash
# 解决方案 1: 清理后重新安装
make clean-all
make setup

# 解决方案 2: 检查网络和 Node 版本
make env-check
node --version  # 需要 >= 20.0.0

# 解决方案 3: 手动安装
cd apps/desktop
rm -rf node_modules
npm install --legacy-peer-deps
```

### 问题：开发服务器无法启动

```bash
# 解决方案 1: 杀掉旧进程
make kill
make dev

# 解决方案 2: 检查端口占用
lsof -ti:1420  # Tauri 端口
lsof -ti:5173  # Vite 端口

# 解决方案 3: 重启并查看详细日志
make dev-debug
```

### 问题：构建失败

```bash
# 解决方案 1: 清理后重新构建
make clean
make build

# 解决方案 2: 完全重置
make clean-all
make setup
make build

# 解决方案 3: 检查 Rust 版本
rustc --version  # 需要 >= 1.70.0
cargo --version
```

### 问题：测试失败

```bash
# 解决方案 1: 运行单个测试
cd apps/desktop/src-tauri
cargo test <test_name> -- --nocapture

# 解决方案 2: 清理测试缓存
make clean-backend
make test-backend

# 解决方案 3: 查看详细输出
cd apps/desktop/src-tauri
RUST_LOG=debug cargo test -- --nocapture
```

### 问题：权限错误

```bash
# 解决方案: 修复权限
make fix-permissions

# 或手动修复
chmod +x apps/desktop/src-tauri/target/release/neuradock
chmod -R u+w apps/desktop/node_modules
```

### 问题：数据库错误

```bash
# 解决方案 1: 重新运行迁移
make migrate

# 解决方案 2: 删除数据库重新创建
rm *.db *.db-shm *.db-wal
make dev  # 会自动创建数据库

# 解决方案 3: 使用开发数据库
# 开发环境会使用 neuradock_dev.db
```

---

## 环境变量

### 日志级别

```bash
# 通过 RUST_LOG 控制日志级别
RUST_LOG=debug make dev      # 详细日志
RUST_LOG=trace make dev      # 追踪级别（最详细）
RUST_LOG=warn make dev       # 仅警告
RUST_LOG=info make dev       # 标准日志（默认）

# 或使用预设命令
make dev-debug               # 相当于 RUST_LOG=debug
make dev-trace               # 相当于 RUST_LOG=trace
make dev-warn                # 相当于 RUST_LOG=warn
```

### 数据库位置

- **开发环境**: `neuradock_dev.db`
- **生产环境**:
  - macOS: `~/Library/Application Support/com.neuradock.app/neuradock.db`
  - Windows: `%APPDATA%\com.neuradock.app\neuradock.db`
  - Linux: `~/.local/share/com.neuradock.app/neuradock.db`

### 日志位置

- macOS: `~/Library/Logs/neuradock/logs/`
- Windows: `%APPDATA%\neuradock\logs\`
- Linux: `~/.local/share/neuradock/logs/`

---

## 性能优化建议

### 加快开发启动速度

```bash
# 1. 使用 dev-fast 跳过依赖检查
make dev-fast

# 2. 使用 Rust 的增量编译（默认启用）
# 已在 Cargo.toml 中配置

# 3. 使用更少的日志
make dev-warn  # 只显示警告
```

### 加快构建速度

```bash
# 1. 仅构建需要的部分
make build-frontend  # 仅前端
make build-backend   # 仅后端

# 2. 使用 Release 快速构建
make build-release-fast  # 编译但不打包

# 3. 使用多核编译
# Rust 默认使用所有 CPU 核心
```

### 减少磁盘占用

```bash
# 1. 定期清理
make clean

# 2. 深度清理（重置环境时）
make clean-all

# 3. 清理 Rust 缓存
cargo cache --autoclean  # 需要安装 cargo-cache
```

---

## 相关文档

- [贡献指南](./contributing.md) - 完整的贡献流程
- [架构概览](./architecture/architecture_overview.md) - 系统架构
- [技术实现细节](./architecture/technical_details.md) - 技术深入文档
- [API 参考](./api/api_reference.md) - API 文档

---

## 获取帮助

```bash
# 查看所有命令
make help

# 查看开发环境状态
make env-check
make status

# 查看版本信息
make version
```

如有问题，请参考：
- [故障排除文档](./user_guide/troubleshooting.md)
- [GitHub Issues](https://github.com/i-rtfsc/NeuraDock/issues)

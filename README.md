# NoAutoStart

Windows 后台自启动进程管理工具

## 功能特性

- **进程列表**: 显示所有运行进程及其详细信息（PID、CPU、内存、启动类型）
- **进程过滤**: 按启动类型、风险等级筛选，支持搜索
- **进程关闭**: 一键关闭进程，支持撤销操作
- **自动关闭列表**: 维护需要开机自动关闭的进程列表
- **历史记录**: 跟踪所有关闭操作，支持恢复
- **系统托盘**: 最小化到托盘，后台运行
- **本地知识库**: 提供常见进程的说明和建议

## 技术栈

- **后端**: Tauri 2.0 (Rust)
- **前端**: Vue 3 + TypeScript
- **构建工具**: Vite
- **状态管理**: Pinia

## 项目结构

```
NoAutoStart/
├── src/                    # Vue 前端
│   ├── components/         # Vue 组件
│   │   └── process/        # 进程相关组件
│   ├── stores/             # Pinia 状态管理
│   ├── types/              # TypeScript 类型定义
│   ├── views/              # 视图组件
│   └── styles/             # CSS 样式
├── src-tauri/              # Tauri/Rust 后端
│   └── src/
│       ├── commands/       # Tauri 命令
│       ├── modules/        # 核心模块
│       │   ├── process_manager.rs   # 进程管理
│       │   ├── config_manager.rs    # 配置管理
│       │   ├── history_manager.rs   # 历史记录
│       │   └── knowledge_manager.rs # 本地知识库
│       ├── tray.rs         # 系统托盘
│       └── lib.rs          # 主入口
└── README.md
```

## 开发

```bash
# 安装依赖
npm install

# 启动开发服务器
npm run tauri dev

# 构建
npm run tauri build
```

## 配置文件位置

- **配置文件**: `%APPDATA%\com.noautostart.app\config.json`
- **历史记录**: `%APPDATA%\com.noautostart.app\history\history.json`
- **知识库**: `%APPDATA%\com.noautostart.app\process_knowledge.json`

## 注意事项

- 部分系统进程无法关闭（如 svchost.exe、explorer.exe）
- 关闭进程操作会记录到历史，支持撤销
- 系统托盘图标需要正确的图标文件

## 许可证

MIT

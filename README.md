# NoAutoStart

Windows 后台自启动进程管理工具

## 功能特性

- 列出所有运行进程并展示详细信息
- 本地知识库提供进程说明
- AI网页跳转查询进程详情
- 关闭进程（支持撤销）
- 永久关闭自启动项
- 维护自动关闭列表
- 开机自动执行关闭

## 技术栈

- **后端**: Tauri (Rust)
- **前端**: Vue 3 + TypeScript
- **构建工具**: Vite
- **状态管理**: Pinia

## 开发

```bash
# 安装依赖
npm install

# 启动开发服务器
npm run tauri dev

# 构建
npm run tauri build
```

## 许可证

MIT

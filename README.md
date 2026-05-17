# Lumina — 4K Wallpaper Desktop Browser

基于 [Wallhaven.cc](https://wallhaven.cc) 的桌面壁纸浏览器，使用 Tauri v2 + Vue 3 构建，macOS 原生风格玻璃态设计。

<p align="center">
  <img src="https://img.shields.io/badge/platform-macOS-lightgrey?logo=apple" />
  <img src="https://img.shields.io/badge/Tauri-v2-ffc131?logo=tauri" />
  <img src="https://img.shields.io/badge/Vue-3.5-4fc08d?logo=vuedotjs" />
  <img src="https://img.shields.io/badge/Rust-1.77-dea584?logo=rust" />
  <img src="https://img.shields.io/badge/license-MIT-blue" />
</p>

## 功能

- **浏览壁纸** — 热门推荐、动漫精选、人物精选三个分类，支持 SFW/Sketchy/NSFW 纯度切换
- **高级筛选** — 排序（Top/Hot/New/Views/Favorites/Random）、分辨率（1080p+ → 8K+）
- **无限滚动** — 自动加载更多壁纸，无需翻页
- **详情灯箱** — 点击壁纸查看大图，支持鼠标滚轮无极缩放、拖拽平移、前后切换
- **一键下载** — 流式下载带进度条，支持自定义下载目录
- **收藏管理** — 本地收藏夹，数据存储在浏览器 localStorage
- **API Key 支持** — 在设置页配置 Wallhaven API Key，解锁 NSFW 内容
- **刷新功能** — 点击刷新按钮重新拉取当前页数据
- **自定义标题栏** — 无原生窗口装饰，macOS 风格自绘标题栏

## 技术栈

| 层 | 技术 |
|---|------|
| 桌面框架 | [Tauri v2](https://v2.tauri.app)（Rust 后端 + WebView 前端）|
| 前端 | Vue 3 + TypeScript + Vite |
| CSS | Tailwind CSS v4 |
| 状态管理 | Pinia |
| 路由 | Vue Router 5 |
| HTTP 客户端 | reqwest（Rust 侧代理所有 API 请求）|
| 图标 | Material Symbols + Lucide |

## 为什么用 Rust 代理请求？

Wallhaven 的 API 有 CORS 限制，图片 CDN（`w.wallhaven.cc`）有防盗链保护（检查 `Referer` 头）。前端无法直接请求。

Rust 后端通过 `reqwest` 代理所有 HTTP 请求：
- **搜索 API** → 转发到 `wallhaven.cc/api/v1/search`，绕过 CORS
- **图片加载** → 下载到本地缓存目录，`Referer` 头设为 `https://wallhaven.cc/`
- **壁纸下载** → 流式下载，每 100ms 向前端推送进度事件

## 快速开始

### 前置条件

- [Node.js](https://nodejs.org) ≥ 18
- [pnpm](https://pnpm.io) ≥ 8
- [Rust](https://www.rust-lang.org) ≥ 1.77.2

### 开发

```bash
# 安装前端依赖
pnpm install

# 启动 Tauri 开发模式（自动启动 Vite + Rust 后端）
cargo tauri dev
```

DevTools 快捷键：`Cmd/Ctrl+J` 打开，`Cmd/Ctrl+K` 关闭（仅 debug 构建）。

### 构建生产版本

```bash
# 构建 .app / .dmg
cargo tauri build

# 仅构建 DMG 并输出到 builder/ 目录
pnpm build:dmg
```

## 项目结构

```
wallhaven-tauri/
├── src/                          # Vue 前端
│   ├── api/wallhaven.ts          # API 层（调用 Tauri invoke）
│   ├── components/
│   │   ├── common/               # EmptyState, SkeletonGrid, ToastViewport
│   │   ├── layout/               # AppShell, Sidebar, Titlebar
│   │   └── wallpaper/            # GlobalHeader, WallpaperCard, WallpaperGrid, WallpaperDetail
│   ├── composables/              # useBrowsePage, useInfiniteScroll, useMouseGlow
│   ├── router/                   # 路由配置（7 个页面）
│   ├── stores/                   # Pinia stores（wallpaper, download, collection, filter, user, toast）
│   ├── types/                    # TypeScript 类型定义
│   └── views/                    # 页面组件（Home, Anime, People, Collection, Download, Settings, About）
├── src-tauri/                    # Rust 后端
│   └── src/
│       ├── lib.rs                # 应用启动、插件注册、命令注册
│       ├── commands.rs           # 所有 Tauri 命令处理器
│       ├── download.rs           # 下载状态管理、流式下载引擎
│       └── main.rs               # Rust 入口点
├── public/                       # 设计稿参考文件
├── package.json
├── vite.config.ts
├── tsconfig.json
└── tailwind.config.ts            # Tailwind 通过 @theme 在 CSS 中配置
```

## 路由

| 路径 | 页面 | 说明 |
|------|------|------|
| `/hot` | 热门推荐 | 全部分类，默认首页 |
| `/anime` | 动漫精选 | 动漫类别 (categories=010) |
| `/people` | 人物精选 | 人物类别 (categories=001) |
| `/collection` | 我的收藏 | 本地收藏夹 |
| `/download` | 下载中心 | 下载管理器 |
| `/settings` | 设置 | API Key、下载目录 |
| `/about` | 关于 | 应用信息 |

## 设计系统

Glassmorphism + macOS Ventura 风格：

- 磨砂玻璃面板：`bg-white/40 backdrop-blur-[40px]` + 圆角边框
- 冷色调（浅蓝/紫色）配合暖橙色强调
- 字体：Inter（正文）、Manrope（标题）、JetBrains Mono（标签）
- 280px 固定左侧导航栏，三组菜单（在线壁纸 / 我的壁纸 / 其他）

## 许可

MIT License

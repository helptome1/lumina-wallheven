# Gallery — UI 重构设计文档

> 基于 `docs/design.md` 的「东方美学 × 未来科技 × 沉浸式画廊」方向，
> 对当前 Tauri + Vue 3 + Tailwind CSS 项目进行全量 UI 重构设计。

---

## 目录

1. [现状分析](#一现状分析)
2. [设计方向定调](#二设计方向定调)
3. [色彩体系重构](#三色彩体系重构)
4. [字体系统](#四字体系统)
5. [动效体系](#五动效体系)
6. [全局氛围层](#六全局氛围层)
7. [组件级重构方案](#七组件级重构方案)
   - [7.1 AppShell](#71-appshell)
   - [7.2 Titlebar](#72-titlebar)
   - [7.3 Sidebar](#73-sidebar)
   - [7.4 SearchBar](#74-searchbar)
   - [7.5 WallpaperCard](#75-wallpapercard)
   - [7.6 WallpaperGrid](#76-wallpapergrid)
   - [7.7 WallpaperDetail（画廊模式）](#77-wallpaperdetail画廊模式)
   - [7.8 CollectionView](#78-collectionview)
   - [7.9 DownloadView](#79-downloadview)
   - [7.10 AboutView](#710-aboutview)
   - [7.11 EmptyState & SkeletonGrid](#711-emptystate--skeletongrid)
8. [实施路线图](#八实施路线图)

---

## 一、现状分析

### 1.1 已有优点（保留）

| 类别 | 当前实现 | 评价 |
|------|----------|------|
| 暗色基调 | `#07071A` 深邃背景 + 紫/青点缀 | 方向正确，高级感基础已具备 |
| 毛玻璃 | `glass` / `glass-elevated` / `glass-card` 三档 | 层次清晰，backdrop-filter 已启用 |
| 极光氛围 | `aurora-bg` 径向渐变背景 | 概念好，但只有静态渐变 |
| 卡片动效 | hover 微缩放 + 渐变遮罩揭示 | 干净克制 |
| 详情灯箱 | 全屏沉浸式预览 + 缩放/旋转控制 | 功能完整 |
| 侧边栏 | 毛玻璃背景 + 活跃指示器 | 结构清晰 |
| 瀑布流 | CSS columns 布局 | 轻量高效 |
| 空状态 | `EmptyState` 组件 | 统一 |

### 1.2 需改进的问题

| 问题 | 严重度 | 说明 |
|------|--------|------|
| 静态背景 | ⭐⭐⭐ | `aurora-bg` 仅做 CSS 渐变，缺乏动态感 |
| 无氛围色联动 | ⭐⭐⭐ | UI 不响应壁纸主色，缺少 Ambient UI |
| 卡片入场动画单调 | ⭐⭐ | 仅 `translateY + scale + opacity`，缺乏 staggered reveal |
| 侧边栏品牌感弱 | ⭐⭐ | 仅有文字 logo，缺乏视觉识别点 |
| 标题栏过于朴素 | ⭐⭐ | 纯功能性，缺乏与整体氛围的呼应 |
| 搜索栏缺乏呼吸感 | ⭐⭐ | 控件紧凑，间距和留白不足 |
| 无噪点纹理层 | ⭐⭐ | 缺乏 Noise Texture 增加质感 |
| 无鼠标视差效果 | ⭐⭐ | design.md 明确提到的 3D 微景深未实现 |
| 收藏页空状态单调 | ⭐ | 仅显示图标 + 文字 |
| 页面切换动效太快 | ⭐ | 250ms 偏快，缺乏"电影级转场" |

---

## 二、设计方向定调

### 核心定位

```
不是“壁纸下载器”，而是“数字艺术沉浸平台”
```

### 设计关键词

| 维度 | 关键词 |
|------|--------|
| 氛围 | 深邃 · 静谧 · 空灵 |
| 视觉 | 毛玻璃 · 半透明 · 发光边缘 · 噪点纹理 |
| 空间 | 留白 · 分层 · 悬浮 · 视差景深 |
| 动效 | 缓慢 · 柔和 · 克制 · 呼吸感 |
| 色彩 | 紫罗兰极光 + 青蓝渐变 + 暖橙点缀 |
| 内容 | 壁纸优先 · UI 退让 · 自动隐藏 |

### 风格锚点

- **Apple TV**：超大预览、内容铺满、UI 隐形
- **Arc Browser**：侧边栏透明、空间感、色彩跟随
- **Linear**：极简控件、精致微交互、克制动效
- **Wallpaper Engine**：动态背景、氛围光

### "三不"原则

- 不塞满界面（大量留白）
- 不炫技动效（轻、慢、柔）
- 不高饱和色彩（70% 极简 + 20% 动效 + 10% 科技元素）

---

## 三、色彩体系重构

### 3.1 底层背景（Deep Space）

```css
--bg-void:      #060612;   /* 最深：黑洞底色 */
--bg-deep:      #0A0A1E;   /* 基准：深邃夜空 */
--bg-surface:   #0E0E28;   /* 表面：卡片基底 */
--bg-elevated:  #14143A;   /* 浮层：毛玻璃之上 */
--bg-overlay:   rgba(6, 6, 18, 0.92); /* 遮罩 */
```

### 3.2 极光主色调（Aurora Accent）

```css
--aurora-violet:  #8B5CF6;  /* 紫罗兰：主色 */
--aurora-indigo:  #6366F1;  /* 靛蓝：过渡 */
--aurora-teal:    #2DD4BF;  /* 青蓝：辅色 */
--aurora-cyan:    #22D3EE;  /* 天青：高亮 */
```

### 3.3 暖色点缀（Warm Accent）

```css
--warm-amber:  #F59E0B;  /* 琥珀：收藏/热度 */
--warm-rose:   #FB7185;  /* 玫瑰：心形/删除 */
```

### 3.4 语义色

```css
--success:  #34D399;
--warning:  #FBBF24;
--error:    #F87171;
```

### 3.5 文字层级

```css
--text-primary:    #EDE9FE;   /* 主文字 */
--text-secondary:  #A5A0C0;   /* 辅助文字 */
--text-tertiary:   #5C5880;   /* 弱化文字 */
--text-disabled:   #3A3660;   /* 禁用 */
```

### 3.6 边框体系

```css
--border-ghost:    rgba(139, 92, 246, 0.04);  /* 几乎不可见 */
--border-subtle:   rgba(139, 92, 246, 0.08);  /* 微妙边界 */
--border-default:  rgba(139, 92, 246, 0.14);  /* 标准边界 */
--border-strong:   rgba(139, 92, 246, 0.25);  /* 强调边界 */
--border-glow:     rgba(139, 92, 246, 0.40);  /* 发光边界 */
```

### 3.7 渐变预设

```css
/* 极光渐变 — 用于 Hero 区域和详情背景 */
--gradient-aurora: linear-gradient(
  135deg,
  #8B5CF6 0%,
  #6366F1 35%,
  #2DD4BF 70%,
  #22D3EE 100%
);

/* 氛围渐变 — 用于卡片 hover 遮罩 */
--gradient-ambient: linear-gradient(
  180deg,
  transparent 0%,
  rgba(6, 6, 18, 0.3) 40%,
  rgba(6, 6, 18, 0.85) 100%
);

/* 发光渐变 — 用于活跃指示器 */
--gradient-glow: linear-gradient(
  180deg,
  #8B5CF6,
  #2DD4BF
);
```

---

## 四、字体系统

### 4.1 字族

```css
--font-display: 'Playfair Display', 'Georgia', 'Noto Serif SC', serif;
--font-body:    'Inter', 'SF Pro Text', system-ui, -apple-system, sans-serif;
--font-mono:    'JetBrains Mono', 'SF Mono', 'Cascadia Code', monospace;
--font-cjk:     'HarmonyOS Sans SC', 'Noto Sans SC', 'PingFang SC', sans-serif;
```

### 4.2 字号层级

| Token | Size | Weight | Use |
|-------|------|--------|-----|
| `text-hero` | 72px | 600 | 画廊模式标题 |
| `text-display` | 48px | 600 | 页面大标题 |
| `text-heading` | 28px | 500 | 区块标题 |
| `text-title` | 18px | 500 | 卡片标题 |
| `text-body` | 14px | 400 | 正文 |
| `text-label` | 12px | 500 | 标签/按钮 |
| `text-caption` | 11px | 400 | 说明文字 |
| `text-micro` | 10px | 500 | 微小标签、badge |

### 4.3 行高

```css
--leading-tight:   1.15;   /* 大标题 */
--leading-normal:  1.5;    /* 正文 */
--leading-relaxed: 1.75;   /* 长文 */
```

### 4.4 字间距

```css
--tracking-tight:   -0.02em;  /* 大标题 */
--tracking-normal:   0;       /* 正文 */
--tracking-wide:     0.04em;  /* 标签 */
--tracking-wider:    0.12em;  /* 微小标签 */
```

---

## 五、动效体系

### 5.1 核心原则

> 慢 · 柔 · 克制 — 200-400ms 为主，偶用 600ms

| 类别 | 时长 | 缓动函数 | 用途 |
|------|------|----------|------|
| 微交互 | 150ms | `cubic-bezier(0.4, 0, 0.2, 1)` | hover/点击反馈 |
| 标准过渡 | 300ms | `cubic-bezier(0.4, 0, 0.2, 1)` | 显示/隐藏/切换 |
| 页面转场 | 400ms | `cubic-bezier(0.22, 0.61, 0.36, 1)` | 路由切换 |
| 氛围呼吸 | 6-8s | `ease-in-out` | 极光漂移/光晕脉动 |
| 详情展开 | 500ms | `cubic-bezier(0.16, 1, 0.3, 1)` | 画廊模式进入 |
| 卡片入场 | 400ms / stagger | `cubic-bezier(0.22, 0.61, 0.36, 1)` | 瀑布流 reveal |

### 5.2 缓动曲线定义

```css
--ease-out-expo:  cubic-bezier(0.16, 1, 0.3, 1);    /* 弹性收尾 */
--ease-out-quint: cubic-bezier(0.22, 0.61, 0.36, 1); /* 平滑减速 */
--ease-in-out:    cubic-bezier(0.4, 0, 0.2, 1);      /* 标准缓动 */
--ease-spring:    cubic-bezier(0.34, 1.56, 0.64, 1); /* 轻微回弹 */
```

### 5.3 动效清单

#### 极光漂移（Aurora Drift）

```css
@keyframes aurora-drift {
  0%   { background-position: 0% 50%; opacity: 0.8; }
  25%  { background-position: 50% 30%; opacity: 1; }
  50%  { background-position: 100% 50%; opacity: 0.9; }
  75%  { background-position: 50% 70%; opacity: 1; }
  100% { background-position: 0% 50%; opacity: 0.8; }
}
/* 应用于 .aurora-bg::before, duration: 8s, ease-in-out, infinite */
```

#### 光晕呼吸（Glow Breathing）

```css
@keyframes glow-breathing {
  0%, 100% { box-shadow: 0 0 40px rgba(139,92,246,0.10); }
  50%      { box-shadow: 0 0 80px rgba(139,92,246,0.18); }
}
/* 应用于 .nav-item.active, duration: 4s */
```

#### 噪点纹理闪烁（Noise Flicker）

```css
@keyframes noise-flicker {
  0%, 100% { opacity: 0.03; }
  50%      { opacity: 0.05; }
}
```

#### 卡片入场（Staggered Card In）

```css
@keyframes card-enter {
  from {
    opacity: 0;
    transform: translateY(32px) scale(0.94);
    filter: blur(4px);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
    filter: blur(0);
  }
}
/* stagger delay: calc(var(--card-index) * 40ms + 80ms) */
```

#### 画廊模式进入（Gallery Enter）

```css
@keyframes gallery-enter {
  from {
    opacity: 0;
    transform: scale(0.92);
    filter: blur(16px);
  }
  to {
    opacity: 1;
    transform: scale(1);
    filter: blur(0);
  }
}
```

#### 页面切换（Page Transition）

```css
.page-enter-active {
  transition: opacity 400ms var(--ease-out-quint),
              transform 400ms var(--ease-out-quint),
              filter 400ms var(--ease-out-quint);
}
.page-enter-from {
  opacity: 0;
  transform: translateY(12px);
  filter: blur(4px);
}
.page-leave-to {
  opacity: 0;
  transform: translateY(-12px);
  filter: blur(4px);
}
```

#### 悬浮光效（Mouse Glow Follower）

跟随鼠标移动的光晕效果，仅在大面积留白区域显示：

```css
.mouse-glow {
  position: fixed;
  width: 600px;
  height: 600px;
  border-radius: 50%;
  background: radial-gradient(circle, rgba(139, 92, 246, 0.06), transparent 70%);
  pointer-events: none;
  transform: translate(-50%, -50%);
  transition: left 0.6s var(--ease-out-quint), top 0.6s var(--ease-out-quint);
  z-index: 0;
}
```

### 5.4 reduced-motion 支持

所有动效均需在 `prefers-reduced-motion: reduce` 时关闭，或通过 App 设置手动开关。

---

## 六、全局氛围层

### 6.1 噪点纹理覆盖层

在整个应用最上层添加 SVG 噪点，模拟胶片质感：

```html
<!-- 放在 AppShell 根节点内，pointer-events: none -->
<div class="noise-overlay" />
```

```css
.noise-overlay {
  position: fixed;
  inset: 0;
  z-index: 9999;
  pointer-events: none;
  opacity: 0.03;
  background-image: url("data:image/svg+xml,..."); /* SVG noise */
  animation: noise-flicker 4s ease-in-out infinite;
}
```

### 6.2 动态极光背景

将 `aurora-bg` 从静态渐变改为多色动画渐变：

```css
.aurora-bg::before {
  content: '';
  position: absolute;
  inset: 0;
  background:
    radial-gradient(ellipse 80% 60% at 20% 20%, rgba(139, 92, 246, 0.10) 0%, transparent 60%),
    radial-gradient(ellipse 60% 70% at 80% 70%, rgba(45, 212, 191, 0.08) 0%, transparent 60%),
    radial-gradient(ellipse 50% 40% at 60% 30%, rgba(99, 102, 241, 0.06) 0%, transparent 60%);
  background-size: 200% 200%;
  animation: aurora-drift 8s ease-in-out infinite;
  pointer-events: none;
  z-index: 0;
}
```

### 6.3 鼠标光晕跟随

在主区域（`<main>`）上添加一个大型径向渐变光斑，跟随鼠标缓慢移动：

```typescript
// composables/useMouseGlow.ts
export function useMouseGlow() {
  const glowX = ref(0)
  const glowY = ref(0)

  function onMouseMove(e: MouseEvent) {
    glowX.value = e.clientX
    glowY.value = e.clientY
  }

  onMounted(() => window.addEventListener('mousemove', onMouseMove, { passive: true }))
  onUnmounted(() => window.removeEventListener('mousemove', onMouseMove))

  return { glowX, glowY }
}
```

```html
<div
  class="mouse-glow"
  :style="{ left: `${glowX}px`, top: `${glowY}px` }"
/>
```

### 6.4 Ambient Color 联动（未来增强）

当选中或预览某张壁纸时，提取其主色并动态更新 CSS 变量，实现 UI 氛围色跟随：

```
壁纸蓝色 → UI 的 --aurora-violet 偏向蓝紫
壁纸橙色 → UI 的 --warm-amber 增强
```

可通过 Canvas 提取图片主色，写入 CSS `style.setProperty()`。

> 此项复杂度较高，建议作为 Phase 2 实施。

---

## 七、组件级重构方案

### 7.1 AppShell

**现状**：固定三区布局（Titlebar + Sidebar + Main），静态极光背景。

**重构目标**：化身"沉浸式容器"，承载全局氛围层。

**方案**：

```
┌──────────────────────────────────────┐
│  .noise-overlay (z-9999, 噪点纹理)    │
│  .mouse-glow    (z-0, 鼠标光晕)       │
│                                      │
│  ┌────┬──────────────────────────┐   │
│  │    │  Titlebar (z-100)        │   │
│  │    ├──────────────────────────┤   │
│  │ S  │                          │   │
│  │ i  │  Main Content (z-10)     │   │
│  │ d  │  + .aurora-bg::before    │   │
│  │ e  │                          │   │
│  │ b  │                          │   │
│  │ a  │                          │   │
│  │ r  │                          │   │
│  └────┴──────────────────────────┘   │
└──────────────────────────────────────┘
```

**改动**：
- 添加上述全局氛围层
- 页面切换过渡时间从 `250ms` → `400ms`，加 `filter: blur()` 渐变
- `aurora-bg` 背景动画化

### 7.2 Titlebar

**现状**：深色毛玻璃 + Gallery 文字 + 窗口三按钮。

**重构目标**：更精致的品牌栏，窗口按钮与氛围融合。

**方案**：

```html
<header class="titlebar">
  <!-- 左侧：应用图标 + 名称 -->
  <div class="titlebar-brand">
    <div class="brand-icon">
      <!-- 极光渐变小方块，轻微呼吸动画 -->
      <div class="brand-gem" />
    </div>
    <span class="brand-name">Gallery</span>
  </div>

  <!-- 中间：当前页面标题（可选） -->
  <span class="titlebar-page-title text-caption text-subtle">热门推荐</span>

  <!-- 右侧：窗口控制 -->
  <div class="titlebar-controls">...</div>
</header>
```

**改动细节**：

| 改动项 | 描述 |
|--------|------|
| brand-gem | 8×8px 圆角方块，`bg-gradient-aurora`，`box-shadow` 发光，呼吸动画 `2.5s` |
| brand-name | `font-display italic`，14px，`--aurora-violet` |
| 窗口按钮 | hover 背景改为 `rgba(139,92,246,0.08)`，关闭按钮 hover 改为 `rgba(251,113,133,0.15)` |
| 整体背景 | 从 `rgba(7,7,26,0.85)` 改为 `rgba(6,6,18,0.8)`，`backdrop-filter: blur(16px)` |
| 分隔线 | `border-bottom` 从 `--border-subtle` 改为 `--border-ghost` |

### 7.3 Sidebar

**现状**：210px 宽，毛玻璃背景，导航列表 + active 指示器。

**重构目标**：增强品牌感、导航项的精致度、增加可折叠/自动隐藏能力。

**方案**：

```html
<aside class="sidebar">
  <!-- 品牌区 -->
  <div class="sidebar-brand">
    <div class="brand-mark">
      <span class="brand-letter">G</span>
    </div>
    <p class="brand-subtitle">IMMERSIVE<br/>GALLERY</p>
  </div>

  <!-- 导航区 -->
  <nav class="sidebar-nav">
    <p class="nav-section-label">
      <span class="nav-section-dot" />
      在线壁纸
    </p>
    <button class="nav-item" :class="{ active }">
      <Flame class="nav-icon" />
      <span class="nav-label">热门推荐</span>
      <span v-if="active" class="nav-indicator" />
    </button>
    ...
  </nav>

  <!-- 底部状态 -->
  <div class="sidebar-footer">
    <div class="status-dot online" />
    <span class="status-text">wallhaven.cc</span>
  </div>
</aside>
```

**改动细节**：

| 改动项 | 当前 | 重构后 |
|--------|------|--------|
| 宽度 | 210px | 220px |
| 品牌区 | "Gallery" + "Wallpaper" 小字 | 极光 G 标识 + "IMMERSIVE GALLERY" 竖排 |
| 分区标题 | 纯文字 `text-tertiary` | 带极光圆点 + letter-spacing: 0.12em |
| 导航项 padding | `9px 12px` | `10px 14px`（更多呼吸空间） |
| 活跃指示器 | 2px 竖线，static 渐变 | 2px 竖线，`box-shadow` 发光 + `glow-breathing` 动画 |
| 导航项 active 背景 | `accent-glow` + `box-shadow` | 同左,但阴影更柔和 |
| 图标透明度 | active 时 1, 默认 0.5 | active 时 1, hover 时 0.8, 默认 0.45 |
| 底部 | "Images from wallhaven.cc" | 绿色状态点 + "wallhaven.cc" |

**可折叠功能（可选）**：

```typescript
const collapsed = ref(false)
// 折叠状态下 sidebar 宽度变为 56px，仅显示图标
// 鼠标 hover 时展开
```

### 7.4 SearchBar

**现状**：毛玻璃横条，purity 切换 + 3 个 select + 搜索输入框。

**重构目标**：更通透、更有呼吸感的筛选栏。

**方案**：

```html
<div class="search-bar">
  <!-- Purity 分段控制器 -->
  <div class="segment-control">
    <button
      v-for="opt in purityOptions"
      :class="{ active: params.purity === opt.value }"
      class="segment-btn"
    >
      {{ opt.label }}
    </button>
  </div>

  <!-- 分隔线 -->
  <div class="search-divider" />

  <!-- 筛选下拉 -->
  <div class="filter-group">
    <FilterSelect v-model="params.sorting" :options="sortingOptions" label="排序" />
    <FilterSelect v-model="params.atleast" :options="resolutionOptions" label="分辨率" />
    <FilterSelect v-model="params.ratios" :options="ratioOptions" label="比例" />
  </div>

  <!-- 弹性空间 -->
  <div class="flex-1" />

  <!-- 搜索框 -->
  <div class="search-input-wrapper">
    <Search class="search-input-icon" />
    <input
      v-model="params.q"
      placeholder="搜索壁纸…"
      class="search-input"
      @keyup.enter="doSearch"
    />
    <div class="search-input-glow" />
  </div>
</div>
```

**改动细节**：

| 改动项 | 当前 | 重构后 |
|--------|------|--------|
| 整体内边距 | `px-6 py-3.5` | `px-8 py-4`（更多呼吸感） |
| purity 按钮 | 通用样式 | 活跃态增加底部微光条 |
| select 样式 | 内联 SVG 箭头 | 提取为独立 `FilterSelect` 组件，含 label |
| 搜索框 | 256px 宽 | 280px，聚焦时 `box-shadow` 发光 + 底部渐变条 |
| 分隔线 | `w-px h-6 bg-white/[0.06]` | 半透明竖线带极光渐变 |

**FilterSelect 组件**：

```html
<div class="filter-select-wrapper">
  <label class="filter-label">{{ label }}</label>
  <select class="filter-select">...</select>
  <ChevronDown class="filter-chevron" />
</div>
```

每个 FilterSelect 宽度约 `120-140px`，hover 时边框微亮。

### 7.5 WallpaperCard

**现状**：圆角卡片 + hover 渐变遮罩 + 收藏/下载按钮 + purity badge。

**重构目标**：更精致的 hover 状态，增加卡片阴影层次。

**方案**：

```html
<div class="wallpaper-card group" @click="openDetail(data)">
  <!-- 外发光环（hover 时显现） -->
  <div class="card-glow-ring" />

  <!-- 缩略图 -->
  <img :src="data.thumbs.large" class="card-image" loading="lazy" />

  <!-- Hover 遮罩层 -->
  <div class="card-overlay">
    <div class="card-overlay-bg" />
    <div class="card-overlay-content">
      <div class="card-info">
        <span class="card-resolution">{{ data.resolution }}</span>
        <span class="card-views">
          <Eye :size="10" /> {{ formatNumber(data.views) }}
        </span>
      </div>
      <div class="card-actions">
        <button class="card-action-btn" :class="{ favorited }" @click.stop="handleFavorite">
          <Heart :size="14" :fill="favorited ? 'currentColor' : 'none'" />
        </button>
        <button class="card-action-btn" @click.stop="handleDownload">
          <Download :size="14" />
        </button>
      </div>
    </div>
  </div>

  <!-- Purity badge -->
  <span v-if="data.purity !== 'sfw'" class="purity-badge" :class="purityClass">
    {{ purityLabel }}
  </span>
</div>
```

**改动细节**：

| 改动项 | 当前 | 重构后 |
|--------|------|--------|
| 圆角 | `rounded-2xl` (16px) | 保持 |
| 边框 | `ring-1 ring-white/[0.05]` | `ring-1 ring-white/[0.04]` |
| hover 缩放 | `scale-[1.04]` | `scale-[1.03]`（更克制） |
| 遮罩渐变 | `from-[#07071A]/95 via-[#0C0C24]/40` | `from-[#060612]/95 via-transparent`（更深沉） |
| 操作按钮 | `backdrop-filter: blur(16px)` 毛玻璃 | 半透明底 + hover 微放大 + 发光 |
| card-glow-ring | 无 | hover 时显现的极光色外发光环 |
| 入场动画 | `translateY(20px) scale(0.96)` | `translateY(32px) scale(0.94) filter:blur(4px)` |

**card-glow-ring**：

```css
.card-glow-ring {
  position: absolute;
  inset: -2px;
  border-radius: 18px;
  background: linear-gradient(135deg, rgba(139,92,246,0.2), rgba(45,212,191,0.15));
  opacity: 0;
  filter: blur(8px);
  transition: opacity 0.4s var(--ease-out-quint);
  pointer-events: none;
  z-index: 15;
}
.group:hover .card-glow-ring {
  opacity: 1;
}
```

### 7.6 WallpaperGrid

**现状**：CSS columns 布局 + 无限滚动 + 骨架屏。

**重构目标**：增加 staggered reveal 动画、优化滚动体验。

**方案**：

```html
<div class="wallpaper-grid-container">
  <div v-if="wallpapers.length > 0" class="grid-wrapper">
    <div class="masonry-grid">
      <div
        v-for="(wallpaper, index) in wallpapers"
        :key="wallpaper.id"
        class="grid-item"
        :style="{ '--card-index': index % 20 }"
      >
        <WallpaperCard :data="wallpaper" />
      </div>
    </div>
  </div>

  <!-- 加载指示器：极光点 -->
  <div v-if="loading && wallpapers.length > 0" class="loading-indicator">
    <div class="aurora-dots">
      <span class="dot" v-for="i in 3" :key="i" :style="{ animationDelay: `${i * 0.15}s` }" />
    </div>
  </div>
</div>
```

**改动细节**：

| 改动项 | 当前 | 重构后 |
|--------|------|--------|
| 滚动条 | 5px 宽，紫色 | 4px 宽，更透明的紫色 |
| 加载指示器 | 旋转圆圈 + "加载更多…" | 3 个极光渐变圆点，依次呼吸 |
| 内边距 | `px-5 pb-8 pt-2` | `px-6 pb-12 pt-4` |
| 入场动画 | `animation-delay: calc(var(--index) * 25ms)` | `calc(var(--card-index) * 40ms + 80ms)`，加 blur 渐变 |

**极光加载点**：

```css
.aurora-dots {
  display: flex;
  gap: 8px;
}
.aurora-dots .dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: linear-gradient(135deg, var(--aurora-violet), var(--aurora-teal));
  animation: dot-breathe 1.4s ease-in-out infinite;
}
@keyframes dot-breathe {
  0%, 100% { opacity: 0.3; transform: scale(0.8); }
  50%      { opacity: 1;   transform: scale(1.2); }
}
```

### 7.7 WallpaperDetail（画廊模式）

**现状**：全屏灯箱，毛玻璃背景，缩放/旋转/翻页控制栏。

**重构目标**：增加"画廊模式"——UI 完全隐退，仅剩壁纸与微弱信息。

**方案**：

```
┌──────────────────────────────────────┐
│  [关闭]                          [信息]  ← 仅两按钮，低透明度
│                                      │
│           全屏壁纸展示                  │
│           （支持缩放/拖拽）              │
│                                      │
│  左下：ID · 分辨率 · 大小（极其微弱）      │
│                                      │
│  ──────────────────────────────────   │
│  底部控制栏（hover 才浮现）              │
│  [←] [缩放-] [100%] [缩放+] [旋转] [♥] [↓] │
└──────────────────────────────────────┘
```

**三层模式切换**：

| 模式 | 触发 | UI 可见度 | 壁纸展示 |
|------|------|-----------|----------|
| 默认 | 点击卡片进入 | 顶部关闭按钮 + 底部工具栏 + 信息面板 | 居中，最大 85vh |
| 画廊模式 | 双击壁纸 / 按 G | 仅剩微弱关闭按钮 | 全屏铺满 |
| 沉浸模式 | 画廊模式下 3 秒不操作 | 全部 UI 淡出 | 全屏铺满 |

**进入动效**：

```css
.detail-fade-enter-active {
  transition: opacity 0.4s var(--ease-out-quint),
              backdrop-filter 0.4s var(--ease-out-quint);
}
.detail-fade-enter-from {
  opacity: 0;
  backdrop-filter: blur(0);
}
```

**画廊模式键盘快捷键**：

| 键 | 操作 |
|----|------|
| `G` | 切换画廊模式 |
| `F` | 全屏（仅壁纸） |
| `I` | 显示/隐藏信息面板 |
| `Space` | 切换自动播放（幻灯片） |
| `1-5` | 缩放级别 |
| `Esc` | 退出画廊 → 默认 → 关闭 |

**控制栏自动隐藏**：

```css
.detail-toolbar {
  opacity: 0;
  transform: translateY(16px);
  transition: opacity 0.3s, transform 0.3s;
}
.detail-root:hover .detail-toolbar,
.detail-root:focus-within .detail-toolbar {
  opacity: 1;
  transform: translateY(0);
}
```

### 7.8 CollectionView

**现状**：标题 + 计数 + WallpaperGrid。

**重构目标**：增加收藏页的策展感，空状态更有吸引力。

**方案**：

有收藏时：

```html
<div class="collection-view">
  <div class="collection-hero">
    <div class="hero-gradient" />
    <h2 class="display-italic text-accent text-[28px]">我的收藏</h2>
    <p class="collection-count">{{ items.length }} 张壁纸 · 精选数字画廊</p>
    <div class="collection-actions">
      <button class="btn-accent-sm">随机播放</button>
      <button class="btn-ghost-sm">导出列表</button>
    </div>
  </div>
  <WallpaperGrid ... />
</div>
```

空收藏时（增强空状态）：

```html
<div class="collection-empty">
  <div class="empty-aurora" />
  <div class="empty-icon-wrapper">
    <Heart class="empty-icon" />
    <div class="empty-icon-glow" />
  </div>
  <h3 class="empty-title">收藏你的灵感</h3>
  <p class="empty-desc">浏览壁纸时点击心形图标，<br/>它们会出现在这里</p>
  <button @click="$router.push('/hot')" class="btn-accent">
    探索热门壁纸 <ArrowRight :size="16" />
  </button>
</div>
```

**空状态动画**：

```css
.empty-icon-glow {
  animation: glow-breathing 3s ease-in-out infinite;
}
.empty-icon {
  animation: heart-draw 2s var(--ease-out-quint);
}
@keyframes heart-draw {
  from { stroke-dashoffset: 100; }
  to   { stroke-dashoffset: 0; }
}
```

### 7.9 DownloadView

**现状**：标题 + 下载中/已完成列表 + 空状态。

**重构目标**：增加进度条的视觉愉悦感，完成状态更有仪式感。

**方案**：

```html
<div class="download-view">
  <div class="download-hero">
    <h2 class="display-italic text-accent text-[28px]">下载中心</h2>
    <p v-if="activeCount" class="download-status">正在下载 {{ activeCount }} 项</p>
  </div>

  <!-- 下载中 -->
  <div class="download-section">
    <p class="section-label">下载中</p>
    <div v-for="item in activeDownloads" class="download-item active">
      <img :src="item.thumbnail" class="item-thumb" />
      <div class="item-info">
        <p class="item-name">{{ item.file_name }}</p>
        <p class="item-meta">{{ formatSize(item.received) }} / {{ formatSize(item.total) }}</p>
        <div class="progress-bar">
          <div class="progress-fill" :style="{ width: `${percent}%` }">
            <div class="progress-shine" />
          </div>
        </div>
      </div>
      <button @click="cancel" class="item-cancel"><X :size="14" /></button>
    </div>
  </div>

  <!-- 已完成 -->
  <div class="download-section">
    <p class="section-label">已完成</p>
    <div v-for="item in completedDownloads" class="download-item done">
      <img :src="item.thumbnail" class="item-thumb" />
      <div class="item-info">
        <p class="item-name">{{ item.file_name }}</p>
        <p class="item-meta">{{ item.resolution }}</p>
      </div>
      <CheckCircle class="item-done-icon" />
    </div>
  </div>
</div>
```

**进度条微动效**：

```css
.progress-bar {
  height: 3px;
  border-radius: 1.5px;
  background: rgba(255,255,255,0.04);
  overflow: hidden;
}
.progress-fill {
  height: 100%;
  border-radius: 1.5px;
  background: linear-gradient(90deg, var(--aurora-violet), var(--aurora-teal));
  position: relative;
  transition: width 0.5s var(--ease-out-quint);
}
.progress-shine {
  position: absolute;
  inset: 0;
  background: linear-gradient(90deg, transparent, rgba(255,255,255,0.15), transparent);
  animation: progress-shine 2s ease-in-out infinite;
}
@keyframes progress-shine {
  from { transform: translateX(-100%); }
  to   { transform: translateX(200%); }
}
```

### 7.10 AboutView

**现状**：居中卡片，Logo + 版本 + 技术栈信息。

**重构目标**：更像产品"身份页"，增加品牌感。

**方案**：

```html
<div class="about-view">
  <div class="about-aurora" />
  <div class="about-card">
    <div class="about-logo">
      <span class="about-logo-letter">G</span>
      <div class="about-logo-glow" />
    </div>
    <h1 class="about-title">Gallery</h1>
    <p class="about-tagline">数字艺术沉浸平台</p>
    <div class="about-divider" />
    <dl class="about-info">
      <div class="about-row">
        <dt>版本</dt>
        <dd>1.0.0</dd>
      </div>
      <div class="about-row">
        <dt>引擎</dt>
        <dd>Tauri 2.x + Vue 3</dd>
      </div>
      <div class="about-row">
        <dt>数据源</dt>
        <dd><span class="text-accent">wallhaven.cc</span></dd>
      </div>
    </dl>
    <p class="about-copyright">壁纸版权归原作者所有 &middot; 仅供个人使用</p>
  </div>
</div>
```

**Logo 动画**：

```css
.about-logo-glow {
  position: absolute;
  inset: -8px;
  border-radius: 24px;
  background: var(--gradient-aurora);
  filter: blur(20px);
  opacity: 0.3;
  animation: glow-breathing 3s ease-in-out infinite;
}
```

### 7.11 EmptyState & SkeletonGrid

**EmptyState**：

| 改动 | 描述 |
|------|------|
| 图标容器 | 增大到 80×80px，添加极光渐变边框 + 微发光 |
| 图标 | `stroke-dasharray` 描边动画，画出图标轮廓 |
| 标题 | 字号 16px，`text-secondary` |
| 描述 | 字号 13px，`text-tertiary`，`max-w-[240px]` |

**SkeletonGrid**：

| 改动 | 描述 |
|------|------|
| 骨架块 | 从单色渐变改为极光色微渐变 |
| 形状 | 模拟真实卡片比例（随机高度 180-320px） |
| 动画 | `shimmer` 动画从 1.8s 变为 2.5s（更慢更柔） |

---

## 八、实施路线图

### Phase 1：基础氛围层（1-2 天）

- [ ] 更新 `style.css` 的 CSS 变量（色彩、字体、缓动）
- [ ] 添加噪点纹理覆盖层（SVG noise）
- [ ] 动态化 `aurora-bg`（animation: aurora-drift）
- [ ] 添加鼠标光晕跟随（useMouseGlow composable）
- [ ] 更新全局滚动条样式

### Phase 2：核心布局 + 导航（1 天）

- [ ] 重构 AppShell（全局氛围层集成）
- [ ] 重构 Titlebar（brand gem + 页面标题）
- [ ] 重构 Sidebar（品牌标识 + 导航项精修）
- [ ] 页面切换动效升级（400ms + blur）

### Phase 3：壁纸浏览体验（1-2 天）

- [ ] 重构 WallpaperCard（card-glow-ring + 遮罩精修）
- [ ] 重构 WallpaperGrid（staggered reveal + 加载指示器）
- [ ] 重构 SearchBar（FilterSelect 组件 + 搜索框发光）

### Phase 4：详情与画廊模式（1-2 天）

- [ ] 重构 WallpaperDetail（三层模式：默认/画廊/沉浸）
- [ ] 控制栏自动隐藏
- [ ] 画廊模式键盘快捷键
- [ ] 进入/退出动效升级

### Phase 5：辅助页面（1 天）

- [ ] 重构 CollectionView（Hero banner + 增强空状态）
- [ ] 重构 DownloadView（极光进度条 + 完成状态）
- [ ] 重构 AboutView（品牌身份页）
- [ ] 重构 EmptyState & SkeletonGrid

### Phase 6：未来增强（Phase 2）

- [ ] Ambient Color 联动（提取壁纸主色）
- [ ] 3D 视差效果（鼠标移动时卡片微偏移）
- [ ] 侧边栏可折叠/自动隐藏
- [ ] 壁纸自动播放（幻灯片模式）
- [ ] 动画开关（`reduced-motion` 设置项）

---

## 附录：CSS 变量完整清单

```css
:root {
  /* Background */
  --bg-void: #060612;
  --bg-deep: #0A0A1E;
  --bg-surface: #0E0E28;
  --bg-elevated: #14143A;
  --bg-overlay: rgba(6, 6, 18, 0.92);

  /* Aurora Accent */
  --aurora-violet: #8B5CF6;
  --aurora-indigo: #6366F1;
  --aurora-teal: #2DD4BF;
  --aurora-cyan: #22D3EE;

  /* Warm Accent */
  --warm-amber: #F59E0B;
  --warm-rose: #FB7185;

  /* Semantic */
  --success: #34D399;
  --warning: #FBBF24;
  --error: #F87171;

  /* Text */
  --text-primary: #EDE9FE;
  --text-secondary: #A5A0C0;
  --text-tertiary: #5C5880;
  --text-disabled: #3A3660;

  /* Borders */
  --border-ghost: rgba(139, 92, 246, 0.04);
  --border-subtle: rgba(139, 92, 246, 0.08);
  --border-default: rgba(139, 92, 246, 0.14);
  --border-strong: rgba(139, 92, 246, 0.25);
  --border-glow: rgba(139, 92, 246, 0.40);

  /* Gradients */
  --gradient-aurora: linear-gradient(135deg, #8B5CF6 0%, #6366F1 35%, #2DD4BF 70%, #22D3EE 100%);
  --gradient-ambient: linear-gradient(180deg, transparent 0%, rgba(6,6,18,0.3) 40%, rgba(6,6,18,0.85) 100%);
  --gradient-glow: linear-gradient(180deg, #8B5CF6, #2DD4BF);

  /* Typography */
  --font-display: 'Playfair Display', 'Georgia', 'Noto Serif SC', serif;
  --font-body: 'Inter', 'SF Pro Text', system-ui, sans-serif;
  --font-mono: 'JetBrains Mono', 'SF Mono', monospace;
  --font-cjk: 'HarmonyOS Sans SC', 'Noto Sans SC', 'PingFang SC', sans-serif;

  --leading-tight: 1.15;
  --leading-normal: 1.5;
  --leading-relaxed: 1.75;

  --tracking-tight: -0.02em;
  --tracking-normal: 0;
  --tracking-wide: 0.04em;
  --tracking-wider: 0.12em;

  /* Easing */
  --ease-out-expo: cubic-bezier(0.16, 1, 0.3, 1);
  --ease-out-quint: cubic-bezier(0.22, 0.61, 0.36, 1);
  --ease-in-out: cubic-bezier(0.4, 0, 0.2, 1);
  --ease-spring: cubic-bezier(0.34, 1.56, 0.64, 1);

  /* Radii */
  --radius-sm: 6px;
  --radius-md: 10px;
  --radius-lg: 16px;
  --radius-xl: 24px;
  --radius-2xl: 32px;

  /* Shadows */
  --shadow-card: 0 1px 0 var(--border-ghost), 0 4px 16px rgba(0, 0, 0, 0.4);
  --shadow-elevated: 0 2px 0 var(--border-subtle), 0 16px 48px rgba(0, 0, 0, 0.6);
  --shadow-glow-violet: 0 0 60px rgba(139, 92, 246, 0.12);
  --shadow-glow-teal: 0 0 60px rgba(45, 212, 191, 0.10);
}
```

---

> 文档生成时间：2026-05-14
> 对应项目：`wallhaven-tauri` (Tauri + Vue 3 + Tailwind CSS)
> 基于设计图：`public/design.png` + `docs/design.md`

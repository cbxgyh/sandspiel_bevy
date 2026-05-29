# sandspiel_bevy

`sandspiel` 的 Rust + Bevy + WGSL 重写版。

## 运行

```powershell
cargo run
```

## 当前已实现

- Bevy 0.18.1 桌面应用入口
- 300x300 沙盒网格
- WGSL 材质渲染
- 旧项目主要元素规则迁移
- 风场工具
- 画笔尺寸切换
- 暂停、重置、撤销
- 启动时的地形/种子引导动画

## 控制

- 鼠标左键拖拽：绘制当前元素 / 推风
- `Space`：暂停或继续
- `Ctrl+Z`：撤销
- `R`：重置
- `1` 到 `5`：切换画笔大小

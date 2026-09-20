<!-- lang:zh-CN -->
# Git Clear

Git Clear 是一款引导式界面的桌面 Git 客户端，使用 Rust 和 egui 构建。它把日常和高级 Git 操作组织成清晰的任务页面——项目、保存改动、远程同步、历史版本、分支、临时收起、合并冲突与设置。核心操作会说明影响范围、能否撤回和风险等级，让不熟悉 Git 命令行细节的人也能更可靠地完成操作。

仓库与产品名称为 **Git Clear**；安装后的系统应用名称为 **Git Agent Clear**，应用窗口标题显示为 **Git Agent**。这三个名称指向同一个软件，不是三个不同版本。

## 主要能力

- 管理多个工作空间与仓库：打开、创建、克隆，多项目标签
- 保存改动：暂存、提交、差异查看、文件时间追溯；危险的丢弃操作会二次确认，并明确说明哪些内容无法恢复
- 远程同步：Fetch / Pull / Push，认证失败、无上游、分叉和冲突场景的分步引导
- 历史版本：提交图谱、搜索、比较、检出、回退、拣选与归档
- 分支与临时收起：本地/远程分支管理，Stash 完整流程
- 冲突与历史整理：三方冲突工具（含可选 AI 建议）、交互式变基
- 高级仓库能力：标签、补丁、Git LFS、Git Flow、子模块与子树、性能基准测试
- 设置与桌面工具：主题、字体、语言、SSH、提交身份、托管账号与可选 AI 模型

## 安装

使用前请先安装 [Git](https://git-scm.com/downloads)，并确认终端中执行 `git --version` 能正常返回版本号。Git Clear 调用本机 Git 完成仓库操作，不会在安装包中重复捆绑 Git。

从 [Releases](https://github.com/awiggy/git-clear-desktop/releases) 下载对应平台的安装包：

- **macOS 11 或更高版本**：`GitAgent-Clear-<版本>-macOS.dmg`，同时支持 Apple Silicon 和 Intel；打开后将应用拖入“应用程序”文件夹
- **Windows x64**：`GitAgent-ClearSetup-<版本>.exe`，安装向导可选择安装路径
- **Linux x86_64**：`GitAgent-Clear_<版本>_amd64.deb`，适用于 Debian / Ubuntu

当前 macOS 安装包使用临时签名，尚未经过 Apple Developer ID 公证。首次打开时如果系统提示无法验证开发者，请在访达中右键应用并选择“打开”；仍被拦截时，可前往“系统设置 → 隐私与安全性”确认打开。请只从本仓库的 Releases 页面下载安装包。

安装后用户数据保存在各平台的标准位置：

```text
Windows: <安装路径>\data
macOS:   ~/Library/Application Support/Git Agent Clear
Linux:   $XDG_DATA_HOME/git-agent-clear 或 ~/.local/share/git-agent-clear
```

通过本仓库 Releases 发布的正式安装包提供“检查更新”，更新来源只指向本仓库；直接执行 `cargo build` 得到的本地开发版本不会启用自更新。

## 构建

需要 Rust stable、Cargo、Git，以及目标系统的图形界面开发依赖。Linux 的依赖安装方式可参考 [构建工作流](.github/workflows/build.yml)。

```bash
cargo test --locked                  # 运行测试
cargo build --release --locked       # 产出 Release 二进制，位于 target/release/
```

## 主题

`theme.json` 是界面默认色彩池。主题令牌使用 HSL 模板，`${c}` 会被替换为选中的强调色。应用优先加载可执行文件旁的 `theme.json`，找不到则回退到内置默认。

在 `theme.json` 旁创建 `theme.local.json` 可以在不改动默认文件的情况下自定义配色：只覆盖声明的键，其余继承。该文件已被 Git 忽略。设置 `GIT_AGENT_THEME` 可选择另一个基础配色文件。

## 发布

推送一个新的 `clear-v<版本>` 标签会自动运行测试、构建三个平台的安装包并发布 GitHub Release。标签版本必须与 `Cargo.toml` 一致，已经发布的标签不要重复使用。

```bash
# 示例：先将 Cargo.toml 和 Cargo.lock 更新为 1.2.2
git tag clear-v1.2.2
git push origin clear-v1.2.2
```

以此方式构建的安装包会启用应用内自更新，更新源为本仓库的 Release 流。

## 许可证

当前仓库尚未声明开源许可证。公开可读不代表自动授予复制、修改或分发代码的权利；后续添加 `LICENSE` 文件后，以该文件中的条款为准。

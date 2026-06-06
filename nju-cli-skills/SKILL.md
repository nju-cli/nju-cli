---
name: nju-cli
description: 南京大学相关操作，比如教务通知，交换生，排名，团委等等。
---

# nju-cli

与南京大学网站交互。

## CLI

优先使用 Codex plugin 内置的 `nju-cli` 二进制：

- macOS/Linux: `plugins/nju-cli/scripts/nju-cli`
- Windows: `plugins/nju-cli/scripts/nju-cli.ps1`

如果当前安装没有内置二进制，再使用系统 PATH 中的 `nju-cli`。

## Subcommands

这里的文件路径是相对skill目录（也就是此SKILL.md所在目录）来的

| 网站                                               | skill                           |
| -------------------------------------------------- | ------------------------------- |
| 教务网：官方通知、校历，比如毕业要求，四六级考试等 | subcommands/academic-affairs.md |
| ehall网上办事大厅：包含课表、培养方案、成绩查询等  | subcommands/ehall.md            |
| 交换生管理                                         | subcommands/exchange-system.md  |
| 南大团委：最新动态、公告通知                       | subcommands/youth-league.md     |

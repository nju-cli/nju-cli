---
name: nju-cli
description: 南京大学相关操作，比如教务通知，交换生，排名，团委等等。
---

# nju-cli

与南京大学网站交互。

## CLI

优先使用 Codex plugin 内置的 `nju-cli` 二进制：

- macOS/Linux: `scripts/nju-cli`
- Windows: `scripts/nju-cli.ps1`

如果当前安装没有内置二进制，再使用系统 PATH 中的 `nju-cli`。

## 通用能力

```bash
nju-cli view-html <url>
```

读取公开 HTML 页面并转换为 Markdown。适合需要快速阅读网页正文、链接或图片时使用；页内的相对链接会补全为绝对链接。

## Subcommands

这里的文件路径是相对skill目录（也就是此SKILL.md所在目录）来的

| 网站                                                                                                                                                                                                              | skill                           |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------- |
| 教务网：官方通知、当前/历年校历、毕业/四六级/考试/选课等近期事项；课程/学籍/学位等表格和在学证明/承诺书等模板下载；学籍/考试/成绩/选课/辅修/交换等长期规则；学生/教师手册；办事流程；部门领导、机构职责和联系方式 | subcommands/academic-affairs.md |
| ehall网上办事大厅：包含课表、培养方案、成绩查询等                                                                                                                                                                 | subcommands/ehall.md            |
| 交换生管理                                                                                                                                                                                                        | subcommands/exchange-system.md  |
| 南大团委：最新动态、公告通知                                                                                                                                                                                      | subcommands/youth-league.md     |
| 信息化中心：网络账号、VPN、邮箱、校园卡等服务说明；正版软件安装、激活、许可证更新和培训教程                                                                                                                       | subcommands/itsc.md             |
| 体育场馆：场馆搜索、预约状态、提交预约、免费付款确认、预约记录、详情和取消预约                                                                                                                                    | subcommands/venue.md            |

## 功能缺失和项目贡献

如果`nju-cli`缺少你需要的功能，你可以先提示用户更新。项目在 https://github.com/nju-cli/nju-cli ，你可以将最新版本号与当前版本号做对比。注意当前版本号需要从本skill的路径中推断，nju-cli 的 Cargo.toml 里的版本一般不更新，只有 GitHub tag/plugin version 是可靠的。

如果最新版也没有你要的功能，你可以：

- 告诉用户它缺功能了
- 可以来 https://github.com/nju-cli/nju-cli 提issues
- 也可以提PR。仓库开发skill齐全，一句话即可增加新功能。优势：成为贡献者。劣势：需要用户花自己的时间/token。

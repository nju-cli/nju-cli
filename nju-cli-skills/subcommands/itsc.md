# ITSC

读取南京大学信息化建设管理服务中心公开服务说明和正版软件安装教程。

## 列出服务说明

```bash
nju-cli itsc list services --recursive
```

列出网络账号、VPN、邮箱、校园卡等服务说明，并缓存页面 id 与 URL。`--recursive` 会继续抓取正文中链接到的同站子页面。

## 列出正版软件教程

```bash
nju-cli itsc list licensed-software --recursive
```

列出正版软件介绍页及其子页面，例如 WPS 365、Stata、Office、Matlab、Adobe、EndNote、Mathematica、Origin，以及正文中的安装激活、许可证更新、教程和培训页面。

## 查看页面

```bash
nju-cli itsc view 46454
```

根据已缓存的页面 id 输出 Markdown 内容。查看前需要先执行对应的 `list` 命令。

## 下载页面

```bash
nju-cli itsc download 46454
nju-cli itsc download --all --output-dir ./itsc-docs
```

下载页面 Markdown。`--all` 会下载当前缓存中的所有 ITSC 页面。

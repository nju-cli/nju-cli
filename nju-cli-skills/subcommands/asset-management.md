# asset-management

南京大学资产管理处公开栏目抓取，覆盖综合新闻、通知公告、规章制度、文件下载、处罚通告、办事指南，以及公开招租公告/公示。

## 查看支持栏目

```bash
nju-cli asset-management columns
```

常用栏目：

- `news`：综合新闻
- `notifications`：通知公告聚合栏目
- `regulations`：规章制度
- `downloads-housing-land`、`downloads-housing`、`downloads-info`、`downloads-lease`、`downloads-projects`：文件下载子栏目
- `penalty-notices`：处罚通告
- `guides`：办事指南
- `lease-announcements`：公开招租公告
- `lease-publicity`：公开招租公示

规章制度和通知公告也提供科室子栏目，先用 `columns` 查看完整 slug。

## 列出文章

```bash
# 列出综合新闻第 1 页，并缓存文章 ID
nju-cli asset-management list news

# 列出通知公告
nju-cli asset-management list notifications

# 拉取公开招租公告下所有文章
nju-cli asset-management list lease-announcements --all
```

`list` 会打印文章 id、日期和标题，并把 id 与 URL 缓存到本地，供 `view` 和 `download` 使用。

## 查看正文

```bash
nju-cli asset-management view guides 220707
nju-cli asset-management view lease-publicity 397550
```

查看前需要先执行对应栏目的 `list` 命令，以缓存文章 ID 与 URL。正文会输出为 Markdown，页内相对链接会补全为绝对链接。

## 下载正文

```bash
# 下载一篇文章；需要先 list
nju-cli asset-management download notifications 400559

# 下载栏目下所有文章
nju-cli asset-management download lease-announcements --all --output-dir ./zcc-lease
```

下载结果是 Markdown 文件。正文中的图片、附件和 PDF player 链接会保留为绝对 URL。

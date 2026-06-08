# 科学技术研究院

读取南京大学科学技术研究院公开文章，支持通知公告、科研动态、公示信息、AI4S 专栏，以及科研项目、工作流程、相关下载、机构设置、政策法规、科研平台、科技成果、学风建设下的所有子栏目。

## 查看支持栏目

```bash
nju-cli scit columns
```

常用栏目：

- `notifications` 通知公告
- `research-news` 科研动态
- `public-info` 公示信息
- `ai` AI4S 专栏
- `research-projects` 科研项目，子栏目如 `research-projects-nsfc`、`research-projects-most`
- `workflow` 工作流程，子栏目如 `workflow-projects`、`workflow-platforms`
- `downloads` 相关下载，子栏目如 `downloads-projects`、`downloads-platforms`
- `institutions` 机构设置，子栏目如 `institutions-leaders`
- `policies` 政策法规，子栏目如 `policies-projects`
- `platforms` 科研平台，子栏目如 `platforms-national`
- `achievements` 科技成果，子栏目如 `achievements-awards`
- `academic-integrity` 学风建设，子栏目如 `academic-integrity-policies`

## 列出文章

```bash
# 列出通知公告第 1 页，并缓存文章 ID
nju-cli scit list notifications

# 列出科研动态
nju-cli scit list research-news

# 列出科研项目下的国家自然科学基金栏目
nju-cli scit list research-projects-nsfc

# 拉取栏目下所有文章
nju-cli scit list public-info --all
```

`list` 默认每页 100 条。可用 `--page` 和 `--page-size` 调整；`--all` 会分页拉取完整栏目。

## 查看文章

```bash
nju-cli scit view notifications [文章ID]
nju-cli scit view research-news [文章ID]
```

查看前需要先执行对应栏目的 `list` 命令，以缓存文章 ID 与 URL。正文会输出为 Markdown，页内相对链接会补全为绝对链接。

## 下载文章

```bash
# 下载一篇文章；需要先 list
nju-cli scit download notifications [文章ID]

# 下载多篇文章
nju-cli scit download research-news [文章ID] [文章ID] [文章ID]

# 下载栏目下所有文章
nju-cli scit download public-info --all --output-dir ./scit-public-info
```

下载结果是 Markdown 文件。文章中嵌入的 PDF player 会在 Markdown 末尾保留 PDF 文件的绝对 URL。

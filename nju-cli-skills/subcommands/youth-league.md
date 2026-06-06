# 南大团委

支持“最新动态”和“公告通知”两个栏目：

- `latest-dynamics` 最新动态
- `announcements` 公告通知

```sh
# 列出最新动态第 1 页，并缓存文章 ID
nju-cli youth-league list latest-dynamics

# 列出公告通知第 1 页，并缓存文章 ID
nju-cli youth-league list announcements

# 指定页码；团委站点固定每页 14 条
nju-cli youth-league list announcements --page 2

# 查看文章正文，输出 Markdown；需要先 list
nju-cli youth-league view announcements [文章ID]
nju-cli youth-league view latest-dynamics [文章ID]
```

如果想要处理大量文章，或从中搜索，可以先下载：

```sh
# 下载一篇文章；需要先 list
nju-cli youth-league download announcements [文章ID]

# 下载多篇文章
nju-cli youth-league download latest-dynamics [文章ID] [文章ID] [文章ID]
```

公告中嵌入的 PDF player 会在 Markdown 末尾保留 PDF 文件的绝对 URL。

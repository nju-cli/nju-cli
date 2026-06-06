# 教务网

```sh
# 输出当前全学年教学校历的页面、PDF 和图片链接；不下载、不转文本
nju-cli academic-affairs calendar

# 列出最近 20 条公告，并缓存公告 ID
nju-cli academic-affairs list

# 列出最近 100 条公告
nju-cli academic-affairs list --page-size 100

# 查看公告正文，输出 Markdown；需要先 list
nju-cli academic-affairs view [公告ID]
```

如果想要处理大量通知，或从中搜索，可以先下载：

```sh
# 下载一条公告；需要先 list
nju-cli academic-affairs download [公告ID]

# 下载多条公告
nju-cli academic-affairs download [公告ID] [公告ID] [公告ID]
```

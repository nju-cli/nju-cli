# graduate-admission

研究生招生网公开栏目抓取，覆盖硕士招生、博士招生、夏令营/推免、港澳台招生和信息公开等栏目。

## 查看支持栏目

```bash
nju-cli graduate-admission columns
```

会输出栏目 slug、中文名和列表页 URL。常用栏目包括：

- `master-guide`：硕士招生：简章目录
- `master-notifications`：硕士招生：硕士最新通知
- `doctoral-guide`：博士招生：简章目录
- `doctoral-notifications`：博士招生：博士最新通知
- `summer-camp-recommendation`：夏令营/推免：最新公告
- `hong-kong-macao-taiwan-guide`：港澳台招生：简章目录
- `hong-kong-macao-taiwan-notifications`：港澳台招生：港澳台最新通知
- `public-notices`：信息公开：公示
- `score-lines`：信息公开：复试基本分数线
- `admission-statistics`：信息公开：往年报考录取统计

## 列表

```bash
nju-cli graduate-admission list master-notifications
nju-cli graduate-admission list doctoral-notifications --all
```

`list` 会打印文章 id、日期和标题，并把 id 与 URL 缓存到本地，供 `view` 和 `download` 使用。

## 查看正文

```bash
nju-cli graduate-admission view doctoral-notifications 836175
```

先执行对应栏目的 `list`，再用文章 id 查看 Markdown 正文。正文里的相对链接会补全为绝对链接，PDF 嵌入会补成附件链接。

## 下载正文

```bash
nju-cli graduate-admission download master-notifications 832811 831962
nju-cli graduate-admission download doctoral-notifications --all
```

默认下载到 nju-cli 缓存目录的 `graduate-admission/<栏目>/` 下；可以用 `--output-dir` 指定目录。

## 单页栏目

```bash
nju-cli graduate-admission page contact-office
nju-cli graduate-admission page contact-schools
```

用于读取“联系招办”“联系学院”这类不是文章列表的页面。

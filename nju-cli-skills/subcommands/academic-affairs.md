# 教务网

## 按场景选择

| 场景                                                                                                   | 使用                         |
| ------------------------------------------------------------------------------------------------------ | ---------------------------- |
| 查近期通知、临时安排、申报/报名/考试/选课/毕业/四六级等有时效性的事项                                  | `notifications`              |
| 查当前全学年校历、校历 PDF 或图片链接                                                                  | `calendar`                   |
| 查历年校历目录，或下载某学年校历 PDF/图片                                                              | `downloads calendar-catalog` |
| 下载课程、学籍、学位、听课评价等教务表格                                                               | `downloads forms`            |
| 下载在学证明、放弃推免资格承诺书等常用教务模板                                                         | `downloads templates`        |
| 查学校层面的长期制度，如学籍管理、考试管理、成绩管理、选课、辅修、缓考、补考重修、交换培养与课程认定等 | `school-regulations`         |
| 查教育部、省厅等上级政策依据，如在线开放课程、转专业、教学指导委员会、创业教育、教学示范中心等         | `ministry-of-edu-doc`        |
| 查学生侧整体规则汇编，不确定具体制度名称时先看学生手册                                                 | `students-manual`            |
| 查教师侧本科教学规则汇编、教学管理要求时看教师手册                                                     | `teachers-manual`            |
| 查某件事具体怎么办、流程如何走、需要什么操作步骤                                                       | `admin-procedures`           |
| 查本科生院部门领导、各机构职责、联系电话、邮箱、负责哪个业务                                           | `institutions`               |

```sh
# 输出当前全学年教学校历的页面、PDF 和图片链接；不下载、不转文本
nju-cli academic-affairs calendar
```

## 下载专区

下载专区在 `downloads` 子命令下，适合获取真实附件文件，而不是把网页正文保存为 Markdown。

包含三个栏目：

- `calendar-catalog`：校历目录，适合查历年校历并下载对应 PDF/图片。
- `forms`：各类表格，适合下载课程、学籍、学位、听课评价等办事表格。
- `templates`：各类模板，适合下载在学证明、承诺书等常用模板。

`list` 默认只列第 1 页并缓存条目 ID；可用 `--page` 翻页，或用 `--all` 拉取完整栏目。`download` 需要先执行对应栏目的 `list`，除非直接使用 `download --all`。

```sh
# 列出历年校历目录，并缓存条目 ID
nju-cli academic-affairs downloads calendar-catalog list

# 列出各类表格；指定页码和每页数量
nju-cli academic-affairs downloads forms list --page 1 --page-size 20

# 拉取全部模板条目并缓存 ID
nju-cli academic-affairs downloads templates list --all

# 下载某个校历条目的实际附件文件；需要先 list
nju-cli academic-affairs downloads calendar-catalog download [条目ID]

# 下载某个表格到指定目录；需要先 list
nju-cli academic-affairs downloads forms download [条目ID] --output-dir ./forms

# 下载多个模板条目
nju-cli academic-affairs downloads templates download [条目ID] [条目ID] [条目ID]

# 下载整个栏目；会批量下载实际附件文件
nju-cli academic-affairs downloads forms download --all --output-dir ./forms
```

## 公告通知

公告通知在 `notifications` 子命令下。`list` 会缓存公告 ID，后续 `view` / `download` 需要先执行 `list`。

```sh
# 列出第 1 页最近 20 条公告，并缓存公告 ID
nju-cli academic-affairs notifications list

# 指定页码和每页数量
nju-cli academic-affairs notifications list --page 2 --page-size 100

# 查看公告正文，输出 Markdown；需要先 list
nju-cli academic-affairs notifications view [公告ID]

# 下载一条公告为 Markdown；需要先 list
nju-cli academic-affairs notifications download [公告ID]

# 下载多条公告
nju-cli academic-affairs notifications download [公告ID] [公告ID] [公告ID]
```

## 机构设置

`institutions list` 会显示部门领导，并列出所有机构。机构列表输出格式为：`机构栏目ID 文章ID 机构名称`。`view` / `download` 可以使用机构栏目 ID 或文章 ID。

```sh
# 显示部门领导，并列出所有机构
nju-cli academic-affairs institutions list

# 查看某个机构详情，输出 Markdown
nju-cli academic-affairs institutions view [机构栏目ID或文章ID]

# 下载某个机构详情为 Markdown
nju-cli academic-affairs institutions download [机构栏目ID或文章ID]

# 下载多个机构详情
nju-cli academic-affairs institutions download [机构栏目ID或文章ID] [机构栏目ID或文章ID]
```

## 规章制度和办事流程

以下栏目都支持同一组 `list` / `view` / `download` 命令：

- `ministry-of-edu-doc`：教育部文件
- `school-regulations`：学校文件
- `students-manual`：学生手册
- `teachers-manual`：教师手册
- `admin-procedures`：办事流程

`list` 默认只列第 1 页并缓存文章 ID；可用 `--page` 翻页，或用 `--all` 拉取完整栏目。`view` / `download` 需要先执行对应栏目的 `list`。

```sh
# 教育部文件：列出第 1 页，每页 100 条
nju-cli academic-affairs ministry-of-edu-doc list

# 学校文件：列出第 2 页，每页 10 条
nju-cli academic-affairs school-regulations list --page 2 --page-size 10

# 学生手册：拉取完整栏目并缓存文章 ID
nju-cli academic-affairs students-manual list --all

# 查看文章正文，输出 Markdown；需要先 list
nju-cli academic-affairs school-regulations view [文章ID]

# 下载一篇文章为 Markdown；需要先 list
nju-cli academic-affairs ministry-of-edu-doc download [文章ID]

# 下载多篇文章
nju-cli academic-affairs school-regulations download [文章ID] [文章ID] [文章ID]

# 下载整个栏目；会批量下载并转换为 Markdown
nju-cli academic-affairs school-regulations download --all

# 指定输出目录
nju-cli academic-affairs admin-procedures download --all --output-dir ./admin-procedures
```

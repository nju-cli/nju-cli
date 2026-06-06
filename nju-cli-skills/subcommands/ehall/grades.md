# ehall：成绩查询

页面进入方式是服务大厅搜索“成绩查询”，选择“成绩查询”。CLI 对应：

## 课程成绩

```sh
# 列出所有学期
nju-cli ehall grades terms

# 列出默认学期范围内的成绩
nju-cli ehall grades list

# 指定学期，支持多次传入
nju-cli ehall grades list --term 2025-2026-2
nju-cli ehall grades list --term 2025-2026-1 --term 2025-2026-2

# 按课程名或课程号筛选
nju-cli ehall grades list --course-name 形势与政策
nju-cli ehall grades list --course-id 00000080H

# 只看及格或未及格课程
nju-cli ehall grades list --passed
nju-cli ehall grades list --failed

# 和页面里的“显示最高成绩”一致
nju-cli ehall grades list --show-max-grade

# 输出完整 JSON
nju-cli ehall grades list --json
```

`list` 的普通输出包含学年学期、课程号、课程名、学分、课程性质、总成绩、是否及格和成绩单标记。

## 四六级成绩

```sh
# 查询大学英语四级、六级成绩
nju-cli ehall grades cet

# 指定学期
nju-cli ehall grades cet --term 2023-2024-1

# 指定考试项目代码；不传默认 CET4 和 CET6
nju-cli ehall grades cet --exam-type CET6

# 输出完整 JSON
nju-cli ehall grades cet --json
```

`cet` 的普通输出包含学年学期、考试项目、成绩、是否通过、考试日期和院系。

## 体测成绩

```sh
# 查询体测成绩
nju-cli ehall grades fitness

# 指定学期
nju-cli ehall grades fitness --term 2025-2026-2

# 输出完整 JSON
nju-cli ehall grades fitness --json
```

`fitness` 的普通输出包含学年学期、考试项目、成绩、是否通过、考试日期和院系。

## 下载

下载会拉取所有匹配成绩，默认写 TSV：

```sh
# 下载默认学期范围内所有成绩到 grades.tsv
nju-cli ehall grades download

# 指定筛选和输出文件
nju-cli ehall grades download --term 2025-2026-2 -o grades.tsv

# 下载 JSON
nju-cli ehall grades download --json -o grades.json

# 下载四六级成绩
nju-cli ehall grades download-cet -o cet-grades.tsv
nju-cli ehall grades download-cet --json -o cet-grades.json

# 下载体测成绩
nju-cli ehall grades download-fitness -o fitness-grades.tsv
nju-cli ehall grades download-fitness --json -o fitness-grades.json
```

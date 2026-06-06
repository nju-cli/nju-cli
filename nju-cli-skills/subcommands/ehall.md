# ehall

ehall 命令需要先登录：

```sh
nju-cli login --username USERNAME --password PASSWORD
```

## 全校本科课表

页面进入方式是服务大厅搜索“课表”，选择“本-课表查询”。CLI 对应：

```sh
# 列出当前学期第 1 页，每页 20 条
nju-cli ehall all-undergraduate-courses list

# 指定页码和页大小
nju-cli ehall all-undergraduate-courses list --page 2 --page-size 50

# 指定学期
nju-cli ehall all-undergraduate-courses list --term 2025-2026-2

# 常用筛选：课程名、教师、校区代码、开课单位代码
nju-cli ehall all-undergraduate-courses list --course-name 高等数学 --teacher 张三 --campus 3 --department 400290

# 任意字段筛选。默认自动推断 builder，也可显式写 FIELD:BUILDER=VALUE
nju-cli ehall all-undergraduate-courses list --filter KCH=00000041 --filter SKJS=王建华
nju-cli ehall all-undergraduate-courses list --filter PKDWDM:m_value_equal=400290

# 输出完整 JSON
nju-cli ehall all-undergraduate-courses list --json --page-size 5
```

下载会拉取所有匹配课程，默认写 TSV：

```sh
# 下载当前学期所有课程到 all-undergraduate-courses.tsv
nju-cli ehall all-undergraduate-courses download

# 指定筛选和输出文件
nju-cli ehall all-undergraduate-courses download --course-name 中国近现代史纲要 -o courses.tsv

# 下载 JSON
nju-cli ehall all-undergraduate-courses download --json -o courses.json
```

常用筛选参数：

- `--course-id` 课程号 KCH
- `--course-name` 课程名 KCM
- `--class-name` 教学班名称 JXBMC
- `--teacher` 上课教师 SKJS
- `--campus` 校区代码 XXXQDM
- `--department` 开课单位代码 PKDWDM
- `--general-category` 通修课程类别代码 TXKCLB
- `--weekday` 上课日期 SKXQ
- `--start-period` 开始节次 KSJC
- `--end-period` 结束节次 JSJC
- `--week` 上课周次 SKZC
- `--building` 教学楼代码 JXLDM
- `--classroom` 上课教室 SKJAS

## 我的课表

页面进入方式是服务大厅搜索“课表”，选择“我的课表”。如果搜索不到入口，先在服务大厅点一次登录；统一认证的 `CASTGC` cookie 不能直接作为 ehall 会话使用。CLI 对应：

```sh
# 列出可选学期
nju-cli ehall my-course-schedule terms

# 输出学期接口的完整 JSON
nju-cli ehall my-course-schedule terms --json

# 列出当前学期我的课表
nju-cli ehall my-course-schedule list

# 列出指定学期课表
nju-cli ehall my-course-schedule list --term 2025-2026-2

# 指定页码和页大小
nju-cli ehall my-course-schedule list --page 1 --page-size 50

# 输出课表接口的完整 JSON
nju-cli ehall my-course-schedule list --term 2025-2026-2 --json
```

`list` 的普通输出包含课程号、课程名、教师、上课时间地点、开课单位、学分、期末信息和备注等摘要字段。

课程详情可用课程号 `KCH` 或教学班 `JXBID` 查询，支持一次查询多个课程：

```sh
# 直接输出课程详情 JSON
nju-cli ehall my-course-schedule detail 00000080H

# 一次查询多个课程
nju-cli ehall my-course-schedule detail 00000080H 22011720S

# 查询指定学期
nju-cli ehall my-course-schedule detail 00000080H --term 2025-2026-2

# 下载详情到目录，每个课程写一个 JSON 文件
nju-cli ehall my-course-schedule detail 00000080H 22011720S -o course-details
```

详情 JSON 会尽量包含接口能拿到的全部课程信息，包括课表行、课程信息、教材信息、选课/教学班信息、期末信息和备注等字段。

页面底部的考试说明、上课冲突说明等内容来自 `.kssm-container`：

```sh
nju-cli ehall my-course-schedule exam-notes
```

免修不免考：

```sh
# 查看所有已申请记录
nju-cli ehall my-course-schedule exemptions

# 只看某个学期的申请记录
nju-cli ehall my-course-schedule exemptions --term 2025-2026-2

# 输出完整 JSON
nju-cli ehall my-course-schedule exemptions --json

# 申请某门课免修不免考，课程参数可以是课程号 KCH 或教学班 JXBID
nju-cli ehall my-course-schedule apply-exemption 00000080H --reason "已自学相关内容"

# 指定学期申请
nju-cli ehall my-course-schedule apply-exemption 00000080H --term 2025-2026-2 --reason "已自学相关内容"
```

`apply-exemption` 会先按页面 JS 的流程检查课程是否可申请、选择对应流程和初始状态，再提交申请。这个命令会实际发起申请，运行前确认课程和理由无误。

## 成绩查询

页面进入方式是服务大厅搜索“成绩查询”，选择“成绩查询”。CLI 对应：

```sh
# 列出页面默认展示的学期
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

四六级成绩：

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

体测成绩：

```sh
# 查询体测成绩
nju-cli ehall grades fitness

# 指定学期
nju-cli ehall grades fitness --term 2025-2026-2

# 输出完整 JSON
nju-cli ehall grades fitness --json
```

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

`list` 的普通输出包含学年学期、课程号、课程名、学分、课程性质、总成绩、是否及格和成绩单标记。
`cet` 和 `fitness` 的普通输出包含学年学期、考试项目、成绩、是否通过、考试日期和院系。

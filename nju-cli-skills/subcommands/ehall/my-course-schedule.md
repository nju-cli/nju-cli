# ehall：我的课表

页面进入方式是服务大厅搜索“课表”，选择“我的课表”。如果搜索不到入口，先在服务大厅点一次登录；统一认证的 `CASTGC` cookie 不能直接作为 ehall 会话使用。

## 学期和课表

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

## 课程详情

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

## 考试说明

页面底部的考试说明、上课冲突说明等内容来自 `.kssm-container`：

```sh
nju-cli ehall my-course-schedule exam-notes
```

## 免修不免考

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

# ehall：全校本科课表

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

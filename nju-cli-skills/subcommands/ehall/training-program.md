# ehall：本科人才培养方案查询

页面进入方式是服务大厅搜索“培养方案”，选择“本-培养方案查询”。CLI 对应：

```sh
# 列出所有专业培养方案
nju-cli ehall training-program list

# 按培养方案名称模糊搜索
nju-cli ehall training-program list --name 计算机

# 按年级、院系代码、修读类型代码筛选
nju-cli ehall training-program list --grade 2026 --department 400290 --study-type 01

# 设置每次请求页大小；命令会自动拉取全部匹配结果
nju-cli ehall training-program list --page-size 200

# 输出完整 JSON
nju-cli ehall training-program list --name 计算机 --json
```

`list` 的普通输出为 TSV 摘要，包含培养方案代码 `PYFADM`、培养方案名称、年级、院系和专业。后续 `detail`、`nodes` 等命令都需要使用 `PYFADM`。

常用筛选参数：

- `--name` 培养方案名称 `PYFAMC`，模糊搜索。
- `--grade` 年级代码 `NJDM`，如 `2026`。
- `--department` 院系代码 `DWDM`。
- `--study-type` 修读类型代码 `XDLXDM`，如 `01`。

## 方案详情

```sh
# 查看某个培养方案详情
nju-cli ehall training-program detail PYFADM

# 输出完整 JSON
nju-cli ehall training-program detail PYFADM --json
```

普通输出包含培养方案代码、名称、年级、院系、专业、修读类型，以及接口返回的培养目标、修读要求、准出要求等文本。

## 课程结构节点

培养方案里的课程要求以节点树组织，每个节点有节点代码 `KZH`：

```sh
# 列出某个培养方案的节点树
nju-cli ehall training-program nodes PYFADM

# 输出完整 JSON
nju-cli ehall training-program nodes PYFADM --json
```

普通输出会显示节点树，并提示：

```text
课程查询: nju-cli ehall training-program courses PYFADM <KZH>
节点详情: nju-cli ehall training-program node-detail PYFADM <KZH>
```

节点摘要包含 `KZH`、节点类型、课程类别、课程数、要求学分等信息；如果节点有修读要求或备注，会显示 `[node-detail有额外信息]`。

## 节点详情

```sh
# 查看某个节点详情
nju-cli ehall training-program node-detail PYFADM KZH

# 输出完整 JSON
nju-cli ehall training-program node-detail PYFADM KZH --json
```

节点详情普通输出包含父节点、节点类型、课程类别、课程数、总学分、至少修读学分、修读要求和备注等字段。

## 节点课程

```sh
# 查看某个节点下的所有课程
nju-cli ehall training-program courses PYFADM KZH

# 输出完整 JSON
nju-cli ehall training-program courses PYFADM KZH --json
```

`courses` 会自动拉取该节点下全部课程。普通输出为 TSV 摘要，包含课程号、课程名、学分、建议修读学期和开课院系。

## 会话说明

该功能默认使用“本科学生组”角色。命令执行前会自动打开应用入口、进入带 `_roleId` 的应用页并切换角色；如果接口返回 302、403 或提示会话异常，通常需要重新执行：

```sh
nju-cli login --username USERNAME --password PASSWORD
```

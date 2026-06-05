# 交换生系统

## 查看项目

```bash
nju-cli exchange-system project list --page-size 10
# 输出: [id] [name]

nju-cli exchange-system project view [id]

# 如果要处理大量项目描述，可先下载
nju-cli exchange-system project download [id1] [id2] ...
```

## 查看通知

通知一般是公示某个项目通过了哪些人。

```bash
nju-cli exchange-system notice list --page-size 10
# 输出: [id] [name]

nju-cli exchange-system notice view [id]

# 如果要处理大量项目通知，可先下载
nju-cli exchange-system notice download [id1] [id2] ...
```

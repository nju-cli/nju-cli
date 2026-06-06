<h1 align="center">NJU-cli | NJU.Skills</h1>

<p align="center">让Agent帮你读通知、看文件、走流程！</p>

<p align="center">
  <img alt="image" width="20%" src="https://github.com/user-attachments/assets/d78d6040-8262-420d-892b-86d7b04afd16" />
</p>

你是否曾疑惑过：

- 我这样选课能毕业吗？
- 我的毕业论文几号交来着？翻翻Q群，找半天
- 四六级啥时候考来着？在哪？通知找不到了……
- 创新大赛报名的ddl是啥时候？我去好多群找不到了

现在，你只需要装上`nju-cli`，然后找个agent问：毕业论文几号交？要交什么？

|                                                      毕业论文ddl什么时候                                                  |                                                      毕业要交什么                                                      |
| :--------------------------------------------------------------------------------------------------------------------: | :--------------------------------------------------------------------------------------------------------------------: |
| <img width="100%" alt="image" src="https://github.com/user-attachments/assets/de0576c9-6f95-4814-bf6a-f31fd13fae43" /> | <img width="100%" alt="image" src="https://github.com/user-attachments/assets/bbee4ab4-f631-4bbe-999d-46b7abecc0fb" /> |


就搞定啦！

## 支持的能力

见[Skill](./nju-cli-skills/SKILL.md)

## 安装

### Codex

cli:

```bash
codex plugin marketplace add https://github.com/nju-cli/codex-marketplace.git
codex plugin add nju-cli@nju-cli
```

app:

1. 在plugin页面，`Built by OpenAI`下拉菜单，添加marketplace：`https://github.com/nju-cli/codex-marketplace.git`
2. 进入marketplace，安装`nju-cli`

### Claude

app:

1. 左侧边栏点击customize
2. personal plugins，点击加号，create plugin，add marketplace
3. 选择 Add from a repository
4. 填入URL：`https://github.com/nju-cli/claude-marketplace`，点击sync
5. 安装`nju-cli`

cli:

```bash
claude plugin marketplace add https://github.com/nju-cli/claude-marketplace
claude plugin install nju-cli@nju-cli
```

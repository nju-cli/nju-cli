---
name: add-new-api
description: 如何让nju-cli适配更多的南大API
---

## 代码实现

一般来说，我们会通过操作浏览器并抓包的方式来寻找接口。

1. clone [browser-harness](https://github.com/browser-use/browser-harness)，或者找到现有的clone
2. 连接到chrome，然后尝试完成一遍用户的操作。用户应该要说清楚在哪个URL进行什么操作，照着做就好。
3. 通过观察chrome抓包内容，找到相关API。比如说在网页上找到关键信息（比如课程名），然后去所有请求里面搜。
4. 通过重发请求并观察响应，找到API所需的最小鉴权header:

- 有的接口可能根本不需要任何header/cookie
- 有的接口可能需要user agent或者accept encoding，但不要求登录态
- 有的可能要求登录态，此时优先考虑通过统一认证登录（见site-authentication skill）

5. 在nju-cli的合适位置，放入api的请求、解析实现

## 文档

我们应该在nju-cli-skills中，为新命令编写文档。

`nju-cli-skills/SKILL.md`：这里是入口，应当写清楚**什么场景下**用什么subcommand，各个subcommand能做什么
`nju-cli-skills/subcommands/xxxx.md`：这里是subcommand的详细文档，要写一些调用example并配上注释。

- 如果有二级subcommand，则需要写清楚什么场景下使用哪些，各个二级subcommand主要能做什么。

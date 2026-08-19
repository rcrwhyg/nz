---
name: nz-git-feature
description: >-
  nz git cadence: review before commit, one commit per feature, amend rules,
  never add the reference tree, batch-push after documentation gates. Use when
  committing, amending, pushing, or when pre-commit / GitHub Actions fails.
---

# Git

细则：`.cursor/rules/04-git.mdc`。用户未明确要求时**不要 commit**。

## 提交前

1. 功能做完（测试全绿、零 warning）
2. 请用户 review；未确认不要 commit
3. `.git/hooks/pre-commit` 必须存在；没有则 `pre-commit install`
4. 确认后恰好一次提交；禁止 `--no-verify`、`--no-gpg-sign`

说明写为什么，前缀如 `docs(agent):`、`feat(info-001):`、`fix(pkt):`。

## amend

仅当 HEAD 就是这一笔、中间没有其他功能提交：

- 用户要求，或钩子改了文件需纳入
- CI 失败：修进同一笔；已 push 则只用 `git push --force-with-lease`

必须新开提交：用户宣告该小功能集完成；后面已有别的功能提交；CI 绿了之后的新问题。

## push

第 1–3 闸：**只本地提交**。文档阶段（抽文档 / skills / spec）全部结束后、第 4 闸编码前，统一 `git push origin main` 一次。第 4 闸起：用户明确要求才 push。

## 禁止

- `git add netw-ib-ox-ag-5.39.0/`
- 无条件 `--force`（不是 lease）
- 提交密钥 / `.env`
- 改 `git config`

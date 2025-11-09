如果你已经从别人的 GitHub 仓库克隆了项目，并进行了修改且上传到自建的 GitHub 仓库，现在需要合并其他人提交的 PR，可以按照以下步骤操作：

---

### 1. 确保你的本地仓库配置正确
首先，检查你的远程仓库配置：
```bash
git remote -v
```
你应该会看到类似以下的输出：
```
origin  https://github.com/你的用户名/你的仓库.git (fetch)
origin  https://github.com/你的用户名/你的仓库.git (push)
upstream https://github.com/原始仓库/原始仓库.git (fetch)
upstream https://github.com/原始仓库/原始仓库.git (push)
```
如果没有配置原始仓库（即你克隆的仓库）为 `upstream`，可以使用以下命令添加：
```bash
git remote add upstream https://github.com/原始仓库/原始仓库.git
```

---

### 2. 获取原始仓库的最新更改
从原始仓库（`upstream`）拉取最新的分支信息：
```bash
git fetch upstream
```

---

### 3. 找到 PR 对应的分支
在原始仓库的 GitHub 页面上，找到你要合并的 PR，记录下 PR 编号和对应的分支名称（例如 `username:feature-branch`）。

---

### 4. 将 PR 分支拉取到本地
使用以下命令将 PR 分支拉取到本地：
```bash
git fetch upstream pull/<PR编号>/head:<本地分支名>
```
例如，如果 PR 编号是 `123`，你可以使用：
```bash
git fetch upstream pull/123/head:pr-123
```

---

### 5. 切换到 PR 分支
切换到刚刚拉取的 PR 分支：
```bash
git checkout pr-123
```

---

### 6. 合并 PR 到你的目标分支
切换到你要合并 PR 的目标分支（例如 `main` 或 `master`），然后合并 PR 分支：
```bash
git checkout main
git merge pr-123
```

---

### 7. 解决冲突（如果有）
如果合并过程中有冲突，Git 会提示你解决冲突。你可以手动编辑冲突文件，然后使用以下命令标记冲突已解决：
```bash
git add <冲突文件>
```

---

### 8. 提交合并结果
解决冲突后，提交合并结果：
```bash
git commit
```

---

### 9. 推送更改到你的 GitHub 仓库
将合并后的更改推送到你的 GitHub 仓库：
```bash
git push origin main
```

---

### 10. 清理本地分支（可选）
如果你不再需要 PR 分支，可以删除它：
```bash
git branch -d pr-123
```

---

### 总结
通过以上步骤，你可以在本地合并其他人提交的 PR，并将其同步到你的 GitHub 仓库。关键点在于：
1. 配置 `upstream` 以跟踪原始仓库。
2. 使用 `git fetch upstream pull/<PR编号>/head:<本地分支名>` 拉取 PR 分支。
3. 解决冲突后，推送更改到你的仓库。

确保在合并前仔细检查代码，以避免引入不必要的问题。

```
git remote add upstream https://github.com/gabdube/native-windows-gui.git
git fetch upstream
git remote -v   
```
* 获取pr
```
git fetch upstream pull/308/head:pr-308
```
```
git remote -v   
```
* 合并
```
git merge pr-308
```

* 删除分支
```
git branch -d 分支名称
```

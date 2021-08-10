---
title = "hello"
author = "author"
date = "2021-08-10 15:03:36"
tags = [""]
categories = [""]
slogan = ""
---

备忘
<!--more-->

### 生成ssh的key
- 检查一下

``` bash
ls -al ~/.ssh
```
- 生成新的秘钥

``` bash
ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
```
- 拷贝秘钥

``` bash
pbcopy < ~/.ssh/id_rsa.pub
```
- 然后加入到仓库的ssh里面，测试

``` bash
ssh -T git@github.com
```

<!--more-->
### 下载仓库
- 克隆代码到本地

``` bash
git clone 仓库的地址
```
### 代码上传
- 合并远程仓库

``` bash
git pull origin 远程分支名称
```
- 添加修改的文件

``` bash
git add 文件名
```
- 提交到本地仓库

``` bash
git commit -m '描述'
```
- 提交到远程仓库

``` bash
git push origin 远程分支名称
```
### 文件操作
- 查看本地文件修改状态

``` bash
git status
```
- 查看文件不同

``` bash
git diff
```
- 撤销对当前工作目录下某些文件的修改

``` bash
git checkout 文件名
```
- 删除文件

``` bash
git rm 文件名
```
### 远程分支
- 查看本地分支

``` bash
git branch
```
- 查看所有分支

``` bash
git branch -a
```
- 查看远程的分支详细情况

``` bash
git remote show origin
```
- 清理本地不存在的远程分支

``` bash
git remote prune origin
```
### 创建分支
- 创建本地分支

``` bash
git checkout -b 本地分支名
```
- 创建对应远程分支的本地分支

``` bash
git checkout -b 本地分支名 origin/远程分支名
```
- 创建远程分支

``` bash
git push origin 远程分支名
```
### 合并分支
- 先切换到目标分支

``` bash
git checkout 目标分支
```
- 合并文件

``` bash
git merge 来源分支
```
- 提交目标远程仓库

``` bash
git push origin 目标分支的远程分支
```
### 合并其它单个文件

``` bash
git checkout 分支名称  绝对路径
```

### 生成多个用户

```bash
ssh-keygen -t rsa -C "yourmail@gmail.com" 
不要一路回车，在第一个对话的时候继续写个名字，生成第二个身份。

ssh-add ~/.ssh/id_rsa_github
ssh-add ~/.ssh/id_rsa_gitlab

ssh -vT git@github.com
ssh -T git@gitlab.com
ssh -T git@bitbucket.org

touch .ssh/config


Host git.company.com
HostName git.company.com  //这里填你们公司的git网址即可
PreferredAuthentications publickey
IdentityFile ~/.ssh/id_rsa_gitlab

```
# nodexe

Is it troublesome to use the node command to start JS every time? Is the built-in packaging of Node too large?

This tool can package `. js ` files into an executable file of only ` 0.4 MB `, automatically calling the system's ` node ` to execute.

As long as you have installed node on your system, it will be smooth and unobstructed.

In the future, it may support automatic download and global sharing of Node.js if the system is not installed.

### How to use it?
1.You need to first use `esbuild` or `bun` or any tool you like to package your project into a single. js file.

2.Run nodexe and pass in a JS file, which will generate an `index` (Linux/Mac OS) or `index. exe` (Windows) executable file in the same level directory as the JS file.
```shell
# nodexe [file]
nodexe index.js
```


# 中文
每次都要用 node 命令来启动 js 很麻烦？Node自带的打包又太大了？

用此工具可以将 `.js` 文件打包成一个仅 `0.4 MB` 的可执行文件，自动调用系统的 `node` 来执行。

只要你系统安装了 node，就可以畅通无阻。

未来可能会支持：如果系统没安装 Node.js 将自动下载，并全局共享。

### 如何使用？

1.你需要先使用 `esbuild` 或 `bun` 或者任意你喜欢的工具，将你的项目打包为单个 .js 文件。

2.运行 nodexe 并传入 js 文件，它会在 js 文件的同级目录生成 `index`(Linux / Mac OS) 或者 `index.exe`(Windows) 可执行文件。
```shell
# nodexe [file]
nodexe index.js
```


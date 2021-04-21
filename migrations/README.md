# sqlx-cli 数据库版本管理工具

## 安装
```shell
# supports all databases supported by SQLx
cargo install sqlx-cli

# only for postgres
cargo install sqlx-cli --no-default-features --features postgres
```

## 用法
> 所有命令都需要提供数据库URL.
> 可以使用`--database-url`命令行选项或通过`DATABASE_URL`在环境中或`.env`当前工作目录中的文件中进行设置来完成此操作.
> 有关更多详细信息, 请运行`sqlx <command> --help`.
 
### 创建和删除数据库 
```shell
sqlx database create
sqlx database drop
```

### 创建并运行迁移

> 创建一个新文件`migrations/<timestamp>-<name>.sql`. 将数据库架构更改添加到此新文件.
```shell
sqlx migrate add <name>
```

> 将正在运行的数据库的迁移历史记录与该`migrations/`文件夹进行比较，并运行所有仍待处理的脚本
```shell
sqlx migrate run
```

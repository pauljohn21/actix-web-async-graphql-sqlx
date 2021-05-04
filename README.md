# actix-web-async-graphql-sqlx

> 基于 actix-web + async-graphql + sqlx + postgresql 的样板工程

## server

> 服务端

### TODOS

- [x] 基于 actix-web async-graphql sqlx 为骨架的整合
- [x] 多环境配置文件读取
- [x] 日志 log4rs
- [x] 分包lib, 准备集成测试  
- [x] async-graphql 错误扩展
- [x] 基于 actix-web 日志中间件 配合 log4rs 记录访问日志 并且单独输出到文件
- [x] 健康检查  health_check & ping
- [x] 配置文件控制开启/关闭  ApolloTracing 插件  
- [x] sqlx日志级别单独控制
- [ ] 用户注册, 密码salt与hash
- [x] 入参校验 基于 validator, 通过自定义方法适配 graphql的result.
- [ ] 用户登录
- [ ] JWT鉴权
- [ ] 修改密码
- [ ] 个人资料更新

## app

> web端

TODOS...

# 鸣谢
[actix-web-async-graphql-rbatis](https://github.com/zzy/actix-web-async-graphql-rbatis)
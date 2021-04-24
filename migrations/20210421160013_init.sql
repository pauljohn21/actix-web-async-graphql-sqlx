-- Add migrations script here
drop table if exists users;
create table users
(
    id             UUID        not null default gen_random_uuid() primary key,
    username       varchar     not null unique,
    email          varchar     not null unique,
    password_hash  varchar     not null,
    full_name      varchar     null,
    bio            varchar     null,
    image          varchar     null,
    email_verified boolean     not null default false,
    active         boolean     not null default true,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT current_timestamp
);

comment on table users is '用户表';
comment on column users.id is '主键';
comment on column users.username is '用户名';
comment on column users.email is '邮箱';
comment on column users.password_hash is '密码哈希值';
comment on column users.full_name is '姓名';
comment on column users.bio is '主键';
comment on column users.image is '头像';
comment on column users.email_verified is '邮箱验证';
comment on column users.active is '活跃用户';
comment on column users.created_at is '创建时间';
comment on column users.updated_at is '更新时间';
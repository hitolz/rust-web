# Rust web 开发-7.redis

本系列文章从以下几个方面学习如何使用 Rust 进行 web 开发。

1. web 框架
2. 数据库/orm
3. config
4. log
5. 线程池
6. kafka
7. redis
8. 打包成 docker 镜像
   ……

---
本篇介绍一下 Rust 如何使用 redis。

Redis 的官方库 https://crates.io/crates/redis

## 添加依赖
```toml
redis = "0.23.3"
```

## 连接 redis

```rust
let client = redis::Client::open("redis://127.0.0.1/")?;
```

## 获取 redis 链接

```rust
let connection = client.get_connection().unwrap();
```

## 执行 redis 命令

```rust
pub fn set_ex(key: String, value: String, seconds: i32) -> bool {
   get_redis_conn()
           .set_ex::<String, String, bool>(key, value, seconds as usize)
           .unwrap_or_else(|err| {
              error!("Failed to set_ex value in Redis: {:?}", err);
              false
           })
}

pub fn get(key: String) -> String {
   get_redis_conn()
           .get::<String, String>(key)
           .unwrap_or_else(|err| {
              error!("Failed to get value from Redis: {:?}", err);
              "".to_string()
           })
}
```
写了两个命令的实现，一个是 set_ex，一个是 get，api 中调用。

```rust

#[get("/set_redis")]
async fn set_redis() -> HttpResponse{
   let result = middleware::redis_client::set_ex("hello".to_string(), "hello".to_string(), 100);
   success(Some(result))
}

#[get("/get_redis")]
async fn get_redis() -> HttpResponse{
   let result = middleware::redis_client::get("hello".to_string());
   success(Some(result))
}
```

测试一下
```bash
curl "http://localhost:8099/set_redis" \
     -H 'token: 1'

curl "http://localhost:8099/get_redis" \
    -H 'token: 1'
```

在 redis 中查询一下
![](http://hitol.blog.cdn.updev.cn/mweb/17012416698132.jpg)


以上代码在 [github](https://github.com/hitolz/rust-web/tree/redis)。

# One (in Rust)

`{include "isXiaoLin/One/README.md"}` （跑

# 配置说明

在 `config` 目录下，每个 JSON 文件即为一个命名空间，文件名即为命名空间名，该文件格式如下:

```json
{
    "host": "https://resource.idawnlight.com/",
    "expiration": 0
}
```

`host` 为要代理的主机，结尾必须加 `/`，`expiration` 为过期时间，以秒为单位。

缓存的数据以一次 SHA256 摘要得到的辨识符为名存于 `cache` 目录下。

程序默认监听 `8088` 端口，需要更改可直接在程序中更改。

# Benchmark

(on a slow machine)

```
> wrk -t12 -c400 -d10s http://isXiaoLin/One/gravatar/612cb477b25c88b436e1bdfcac2a8588
Running 10s test @ http://isXiaoLin/One/gravatar/612cb477b25c88b436e1bdfcac2a8588
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    65.68ms   48.48ms 418.71ms   74.46%
    Req/Sec   490.78    195.24     1.32k    78.33%
  48111 requests in 10.10s, 98.88MB read
Requests/sec:   4764.37
Transfer/sec:      9.79MB

> wrk -t12 -c400 -d10s http://idawnlight/one-rust/gravatar/612cb477b25c88b436e1bdfcac2a8588
Running 10s test @ http://idawnlight/one-rust/gravatar/612cb477b25c88b436e1bdfcac2a8588
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    51.09ms    9.23ms 128.20ms   85.29%
    Req/Sec   643.85     81.95     1.60k    87.25%
  76995 requests in 10.08s, 1.01GB read
Requests/sec:   7638.14
Transfer/sec:    102.99MB
```

# Tips

- 小心更多的 Bug (((
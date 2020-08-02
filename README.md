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

```
> wrk -t12 -c400 -d10s http://isXiaoLin/One/gravatar/612cb477b25c88b436e1bdfcac2a8588
Running 10s test @ http://isXiaoLin/One/gravatar/612cb477b25c88b436e1bdfcac2a8588
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   416.14ms  149.20ms   1.96s    86.25%
    Req/Sec    82.58     62.97   410.00     74.36%
  9171 requests in 10.09s, 19.18MB read
Requests/sec:    908.63
Transfer/sec:      1.90MB

> wrk -t12 -c400 -d10s http://idawnlight/one-rust/gravatar/612cb477b25c88b436e1bdfcac2a8588
Running 10s test @ http://idawnlight/one-rust/gravatar/612cb477b25c88b436e1bdfcac2a8588
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    53.68ms    9.91ms 138.54ms   84.02%
    Req/Sec   613.73     74.93     1.16k    82.93%
  73337 requests in 10.05s, 0.97GB read
Requests/sec:   7297.76
Transfer/sec:     98.40MB
```

# Tips

- 小心更多的 Bug (((
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

`host` 为要代理的主机，结尾必须加 /，`expiration` 为过期时间，以秒为单位。

缓存的数据以一次 SHA256 摘要得到的辨识符为名存于 `cache` 目录下。

程序默认监听 `8088` 端口，需要更改可直接在程序中更改。

# Tips

- 小心更多的 Bug (((
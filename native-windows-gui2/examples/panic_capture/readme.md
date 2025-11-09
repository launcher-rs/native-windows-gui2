## panic 捕获

通过 panic::catch_unwind 捕获 panic
通过 panic::set_hook 注册一个自定义panic钩子函数，替换之前注册的任何函数，当发生panic(且被panic::catch_unwind包裹时)时触发 set_hook触发此函数

## 注意
在 `Cargo.toml`中配置函数，可时 panic::catch_unwind+panic::set_hook不生效

```
[profile.release]
# 不生效
panic = 'abort'

[profile.dev]
# 生效
panic = 'unwind'

```
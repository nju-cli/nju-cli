# ddddocr 通用验证码识别 (rust version)

本 crate 基于 https://github.com/sunnysab/d4ocr-rust ，为 nju-cli 的统一认证验证码识别做了少量本地修改。

使用方法:

`cargo run --release --example recognize captcha/captcha.jpg`

经过测试，该命令在 Debug 模式下，单张小验证码需要 8s 左右识别。 在 Release 模式下仅需 200ms。

抛开加载模型的影响，使用如下代码测试：

```rust
let start = Instant::now();
let result = model.recognize(image).unwrap();

println!("{}", start.elapsed().as_millis());
println!("{result}");
```

识别样例中的单张验证码，耗时在 40ms 左右。

## 参考资料

1. ONNX 模型来源 https://github.com/sml2h3/ddddocr （原作者）

2. ONNX 模型调用参考 https://github.com/recoai/visual-search

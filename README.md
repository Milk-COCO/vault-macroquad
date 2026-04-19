# vaultquad
[English Readme](README-en.md)

对的，`vaultquad` 是 [`macroquad`](https://github.com/not-fl3/macroquad) 的个人用的小分支，按照我想怎么改就怎么改的原则爆改了macroquad。很烂，所以不太可能会喂回macroquad上流。

# 我做了啥
主要的：
- 将所有传递坐标/大小的函数改为传入 `impl Into<(f32,f32)>`。在任何地方!
- 重写UI mod，直接把[flowquad](https://github.com/Muhtasim-Rasheed/flowquad)内建并爆改。[在这!](src/ui/mod.rs)
- 自己写了个安全的栈场景系统。[在这!](src/scene.rs)
- 接入了我自己写的ime preedit&commit事件。

# 我的miniquad呢？！
噢想要运行你需要在同级目录下也clone一个我fork的miniquad
```bash
cd ..
git clone https://github.com/Milk-COCO/vault-miniquad
```

或者你可以去`Cargo.toml`改一下配置QwQ
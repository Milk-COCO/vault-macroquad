# vaultquad
[中文读我](README.md)

As you could see, `vaultquad` is my personal [`macroquad`](https://github.com/not-fl3/macroquad) little branch, following the principle of doing what the foobar I want, has significant edited the macroquad. It's really terrible, so I'm unlikely to feedback this any to the upstream.

# What have I done
major:
- Change all functions that transfer coordinates/sizes to pass in 'impl Into<(f32, f32)>'. Anywhere!
- Rewrite the UI mod with modified [flowquad](https://github.com/Muhtasim-Rasheed/flowquad). [Here!](src/ui/mod.rs)
- Write a stack scene system. [Here!](src/scene.rs)
- Connected to the ime preedit&comment event.

# Where is my miniquad?!
Oh, to run it, you need to clone miniquad I forked in the same directory.
```bash
cd ..
git clone https://github.com/Milk-COCO/vault-miniquad
```

Or you can go to `Cargo. toml` to modify the configuration. QwQ

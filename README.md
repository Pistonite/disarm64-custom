# disarm64-custom

This is a custom built version of https://github.com/kromych/disarm64 using the documentation
in that repo, to use with my own projects

Changes:
- Filtered down the memonics to only what I need
- Split the decoder into multiple files to take advantage of parallelism in compilation

## Speed Boost
Using nightly channel, you can set the `-Z thread=n` flag to use parallel HIR lowering
to compile 2x faster:

```toml
# .cargo/config.toml
[build]
rustflags = ["-Z", "thread=4"]
```

## Add
```
cargo add disarm64-custom --git https://github.com/Pistonite/disarm64-custom --branch main
```

## Generate
Tweak `menmonics.txt` and parameters in `make.py`, then run `python make.py`
## Run

```bash
cd compress-dir-as-gz
cargo run
```

You can check the results:

```bash
tar -xvf /tmp/archive.tar.gz -C /tmp/
cat /tmp/result/files/*
```

## Resources

- How to compress a directory: <https://rust-lang-nursery.github.io/rust-cookbook/compression/tar.html#working-with-tarballs>


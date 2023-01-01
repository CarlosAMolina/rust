## Run

```bash
cd compress-file-as-gz
cargo run
```

You can check the results:

```bash
cd /tmp/
gunzip foo.txt.gz
cat foo.txt
```

## Resources

- How to compress with gz: <https://docs.rs/flate2/latest/flate2/write/struct.GzEncoder.html>

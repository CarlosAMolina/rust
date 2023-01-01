## Run

```bash
cd compress-file-as-gz
cargo run
```

You can check the results:

```bash
gunzip /tmp/foo.txt.gz -c
```

## Resources

- Work with gz encoder: <https://docs.rs/flate2/latest/flate2/write/struct.GzEncoder.html>

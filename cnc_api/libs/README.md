Sometimes connecting to the postgres database will fail, something to do with broken dlls
To fix it run these command this should fix it according to https://github.com/diesel-rs/diesel/discussions/2947#discussioncomment-2025857
```bash
cargo build
cp libintl-9.dll ../target/debug/
```

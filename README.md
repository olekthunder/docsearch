# Docsearch (demo)

```
cargo build
mkdir .index
export DOCSEARCH_DOC_DIR="/my/doc/dir"
./target/debug/indexer
./target/debug/search "search term"
```
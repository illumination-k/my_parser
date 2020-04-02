# README

pythonのioが遅くてイライラしたので作成中。

## Usage

### 1. build

mac
```
cargo rustc --release -- -C link-arg=-undefined -C link-arg=dynamic_lookup
```

linux
```
rustup run nightly cargo build --release
```

### 2. make symlink

mac
```
ln -sfv ./target/release/libmy_parser.dylib my_parser.so
```

linux
```
ln -sfv ./target/release/libmy_parser.so my_parser.so
```

### docs

そのうち作る

- hello -> "Hello, World!"
- read_result_csv -> umi_tools countあたりの出力を早く読める
- read_fasta -> fastaを読める(返り値は{id:seq}の辞書)

```python
>>> import my_parser
>>> my_parser.hello()
Hello, World!
```



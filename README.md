# cargo-pystubgen

`cargo-pystubgen` は、Rust で書かれた Python 拡張モジュールに対して、既存の `#[pufunction]` などのアトリビュートを**書き換えることなく非破壊で** `.pyi` スタブファイルを生成する Cargo サブコマンドです。

## ✨ 特徴

- 既存コードを変更せず `.pyi` ファイルを生成
- `#[pufunction]` などのアトリビュートを解析
- 関数定義と一部の型情報に対応
- [uv](https://github.com/astral-sh/uv) のワークスペースおよび単体プロジェクトに対応

## 📦 インストール

```bash
cargo install cargo-pystubgen

## 使用方法
```bash
cargo pystubgen
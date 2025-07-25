# ローカル開発環境セットアップガイド

このガイドでは、NBA Trade Trackerプロジェクトのローカル開発環境をセットアップする手順を説明します。

## 前提条件

- macOS（Apple Silicon推奨）
- Homebrew
- Git

## Rustのセットアップ

### 1. rustupのインストール

プロジェクトではrustupを使用してRustのバージョンを管理します。

```bash
# rustupの公式インストーラーを使用
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# インストール後、設定を反映
source $HOME/.cargo/env
```

### 2. Rust 1.88.0以上のインストール

このプロジェクトはCargo.lock v4を使用しているため、Rust 1.78以上が必要です。

```bash
# 最新のstableバージョンをインストール
rustup update stable

# デフォルトのツールチェーンをstableに設定
rustup default stable

# バージョンを確認
cargo --version  # 1.88.0以上であることを確認
rustc --version  # 1.88.0以上であることを確認
```

### 3. PATHの設定

zshを使用している場合、`~/.zshrc`に以下を追加します：

```bash
### Rust
#### Rustupのバイナリを優先的に使用
export PATH=$HOME/.cargo/bin:$PATH
```

bashを使用している場合は、`~/.bashrc`に同様の設定を追加してください。

### 4. Homebrewのrustとの競合を避ける

Homebrewでrustをインストールしている場合は、アンインストールすることを推奨します：

```bash
# Homebrewのrustがインストールされているか確認
brew list | grep rust

# インストールされている場合はアンインストール
brew uninstall rust
```

## 開発ツールのインストール

### 必須ツール

```bash
# Clippyのインストール（既にrustupでインストール済みの場合が多い）
rustup component add clippy

# rustfmtのインストール（既にrustupでインストール済みの場合が多い）
rustup component add rustfmt
```

### SQLiteのセットアップ（ローカル開発用）

```bash
# macOSの場合（Homebrewを使用）
brew install sqlite3

# 確認
sqlite3 --version
```

## プロジェクトのセットアップ

### 1. リポジトリのクローン

```bash
git clone https://github.com/toguri/iso-flow.git
cd iso-flow
```

### 2. バックエンドのセットアップ

```bash
cd backend

# 環境変数ファイルの作成
cp .env.example .env

# 依存関係のインストール
cargo build

# データベースマイグレーションの実行
cargo run --bin migrate

# 開発サーバーの起動
cargo run
```

### 3. コード品質チェック

```bash
# フォーマットチェック
cargo fmt --check

# Clippyでのリントチェック
cargo clippy -- -D warnings

# テストの実行
cargo test
```

## トラブルシューティング

### Cargo.lock v4エラー

以下のようなエラーが発生した場合：

```
error: failed to parse lock file at: /path/to/Cargo.lock
Caused by:
  lock file version `4` was found, but this version of Cargo does not understand this lock file
```

解決方法：
1. rustupを使用してRust 1.78以上をインストール
2. PATHが正しく設定されていることを確認
3. `which cargo`で`~/.cargo/bin/cargo`が使用されていることを確認

### RLSコンポーネントエラー

rustupの更新時に以下のようなエラーが発生した場合：

```
error: component 'rls-preview' for target 'aarch64-apple-darwin' is unavailable
```

解決方法：
```bash
# RLSとrust-analysisを削除
rustup component remove rls rust-analysis

# その後、rustupを更新
rustup update stable
```

## 推奨されるVSCode拡張機能

- rust-analyzer
- Even Better TOML
- crates

## 参考リンク

- [Rust公式サイト](https://www.rust-lang.org/)
- [rustupドキュメント](https://rust-lang.github.io/rustup/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
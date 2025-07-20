.PHONY: test test-unit test-integration coverage coverage-html clean

# 全テスト実行
test:
	cargo test --all-features

# 単体テストのみ実行
test-unit:
	cargo test --lib --all-features

# 統合テストのみ実行
test-integration:
	cargo test --test '*' --all-features

# カバレッジ測定（テキスト形式）
coverage:
	cargo llvm-cov test --all-features --workspace

# カバレッジ測定（HTML形式）
coverage-html:
	cargo llvm-cov test --all-features --workspace --html
	@echo "カバレッジレポートが生成されました: target/llvm-cov/html/index.html"

# カバレッジデータのクリーンアップ
clean:
	cargo llvm-cov clean --workspace

# ドキュメント生成
docs:
	cargo doc --no-deps --all-features --open

# ドキュメント生成（依存関係含む）
docs-all:
	cargo doc --all-features --open
.PHONY: test coverage coverage-html clean

# テスト実行
test:
	cargo test --all-features

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
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
	cargo tarpaulin --out Stdout --verbose --workspace --exclude-files "*/migrations/*" --exclude-files "*/tests/*"

# カバレッジ測定（HTML形式）
coverage-html:
	cargo tarpaulin --out Html --verbose --workspace --exclude-files "*/migrations/*" --exclude-files "*/tests/*"
	@echo "カバレッジレポートが生成されました: tarpaulin-report.html"

# カバレッジ測定（XML形式 - Codecov用）
coverage-xml:
	cargo tarpaulin --out Xml --verbose --workspace --exclude-files "*/migrations/*" --exclude-files "*/tests/*"
	@echo "カバレッジレポートが生成されました: cobertura.xml"

# カバレッジデータのクリーンアップ
clean:
	rm -f tarpaulin-report.html cobertura.xml
	cargo clean

# ドキュメント生成
docs:
	cargo doc --no-deps --all-features --open

# ドキュメント生成（依存関係含む）
docs-all:
	cargo doc --all-features --open
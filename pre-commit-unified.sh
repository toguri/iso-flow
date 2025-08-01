#!/bin/bash
# Git pre-commit hook for iso-flow project

# プロジェクトルートディレクトリを取得
PROJECT_ROOT=$(git rev-parse --show-toplevel)

# 現在のディレクトリを保存（最後に戻るため）
CURRENT_DIR=$(pwd)

# 変更されたファイルを取得
CHANGED_FILES=$(git diff --cached --name-only)

# backendの変更をチェック
BACKEND_CHANGED=false
if echo "$CHANGED_FILES" | grep -q "^backend/"; then
    BACKEND_CHANGED=true
fi

# frontendの変更をチェック  
FRONTEND_CHANGED=false
if echo "$CHANGED_FILES" | grep -q "^frontend/"; then
    FRONTEND_CHANGED=true
fi

# Docker Composeが存在し、バックエンドの変更がある場合はPostgreSQLを起動
if [ "$BACKEND_CHANGED" = true ] && [ -f "$PROJECT_ROOT/docker-compose.yml" ]; then
    echo "🐳 Starting Docker services for database tests..."
    
    # Docker Composeを起動（すでに起動している場合は何もしない）
    docker-compose -f "$PROJECT_ROOT/docker-compose.yml" up -d postgres
    
    # PostgreSQLが起動するまで待機（最大30秒）
    echo "⏳ Waiting for PostgreSQL to be ready..."
    for i in {1..30}; do
        if docker-compose -f "$PROJECT_ROOT/docker-compose.yml" exec -T postgres pg_isready -U postgres >/dev/null 2>&1; then
            echo "✅ PostgreSQL is ready!"
            break
        fi
        if [ $i -eq 30 ]; then
            echo "❌ PostgreSQL failed to start within 30 seconds"
            exit 1
        fi
        sleep 1
    done
    
    # テスト用のデータベースURLを設定
    export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/test_iso_flow"
    echo "📊 DATABASE_URL set for tests: $DATABASE_URL"
fi

# Backend checks
if [ "$BACKEND_CHANGED" = true ]; then
    echo "🔍 Running pre-commit checks for backend..." >&2
    cd "$PROJECT_ROOT/backend" || exit 1

    # 1. cargo fmt check
    echo "📝 Checking formatting..."
    if ! cargo fmt -- --check; then
        echo "⚠️  Formatting issues found. Running cargo fmt..."
        cargo fmt
        # フォーマットされたファイルを自動的にステージングに追加
        git add -u
        echo "✅ Formatting fixed and staged automatically."
    fi

    # 2. cargo clippy
    echo "🔎 Running clippy..."
    if ! cargo clippy -- -D warnings; then
        echo "❌ Clippy warnings found. Please fix them before committing."
        exit 1
    fi

    # 3. cargo test (including ignored tests)
    echo "🧪 Running all tests (including database tests)..."
    if ! cargo test -- --include-ignored; then
        echo "❌ Tests failed. Please fix them before committing."
        exit 1
    fi

    echo "✅ Backend checks passed!"
fi

# Frontend checks
if [ "$FRONTEND_CHANGED" = true ]; then
    echo "🔍 Running pre-commit checks for frontend..." >&2
    cd "$PROJECT_ROOT/frontend" || exit 1
    
    # npmベースのテストに変更（より一般的）
    if [ -f "package.json" ] && grep -q "\"test\"" package.json; then
        echo "🧪 Running frontend tests..."
        if ! npm test -- --watchAll=false; then
            echo "❌ Frontend tests failed. Please fix them before committing."
            exit 1
        fi
    else
        # Gradleベースのプロジェクトの場合
        if [ -f "./gradlew" ]; then
            echo "🧪 Running frontend tests with Gradle..."
            if ! ./gradlew jsTest; then
                echo "❌ Frontend tests failed. Please fix them before committing."
                exit 1
            fi
        fi
    fi
    
    echo "✅ Frontend checks passed!"
fi

# 元のディレクトリに戻る
cd "$CURRENT_DIR"

echo "✅ All checks passed!"
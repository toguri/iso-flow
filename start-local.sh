#!/bin/bash
echo "🚀 ローカル環境を起動します..."

# バックエンドが起動しているか確認
if ! docker ps | grep -q iso-flow-backend; then
    echo "📦 バックエンドを起動中..."
    docker-compose up -d
    echo "⏳ バックエンドの起動を待機中..."
    sleep 5
else
    echo "✅ バックエンドは既に起動しています"
fi

# フロントエンドを起動
echo "🎨 フロントエンドを起動中..."
cd frontend
./gradlew jsBrowserDevelopmentRun &
FRONTEND_PID=$!
echo "フロントエンドPID: $FRONTEND_PID"

# 起動を待つ
echo "⏳ サービスの起動を待機中..."
sleep 10

# 統合テストを実行
cd ..
./test-integration.sh

echo ""
echo "💡 開発環境が起動しました！"
echo "   フロントエンド: http://localhost:8080"
echo "   バックエンド: http://localhost:8000"
echo ""
echo "停止するには: kill $FRONTEND_PID && docker-compose down"
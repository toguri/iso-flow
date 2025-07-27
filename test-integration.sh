#!/bin/bash
echo "🧪 統合テスト開始..."

# バックエンドのヘルスチェック
echo "📡 バックエンドの確認..."
BACKEND_HEALTH=$(curl -s http://localhost:8000/health)
if [[ $BACKEND_HEALTH == *"healthy"* ]]; then
    echo "✅ バックエンド: 正常"
else
    echo "❌ バックエンド: エラー"
    exit 1
fi

# GraphQLエンドポイントの確認
echo "📊 GraphQLエンドポイントの確認..."
GRAPHQL_RESPONSE=$(curl -s -X POST http://localhost:8000/ \
    -H "Content-Type: application/json" \
    -d '{"query": "{ __typename }"}')
if [[ $GRAPHQL_RESPONSE == *"data"* ]]; then
    echo "✅ GraphQL: 正常"
else
    echo "❌ GraphQL: エラー"
    exit 1
fi

# フロントエンドの確認
echo "🎨 フロントエンドの確認..."
FRONTEND_RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8080)
if [[ $FRONTEND_RESPONSE == "200" ]]; then
    echo "✅ フロントエンド: 正常"
else
    echo "❌ フロントエンド: エラー (HTTP $FRONTEND_RESPONSE)"
    exit 1
fi

# CORS設定の確認
echo "🔒 CORS設定の確認..."
CORS_HEADER=$(curl -s -I http://localhost:8000/ \
    -H "Origin: http://localhost:8080" | grep -i "access-control-allow-origin")
if [[ $CORS_HEADER == *"http://localhost:8080"* ]] || [[ $CORS_HEADER == *"*"* ]]; then
    echo "✅ CORS: 正常"
else
    echo "❌ CORS: エラー"
    echo "   ヘッダー: $CORS_HEADER"
fi

echo ""
echo "🎉 統合テスト完了！"
echo ""
echo "📱 フロントエンド: http://localhost:8080"
echo "🔧 GraphQL Playground: http://localhost:8000"
echo ""
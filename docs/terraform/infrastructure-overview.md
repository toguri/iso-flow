# インフラストラクチャ概要

NBA Trade TrackerのAWSインフラストラクチャ構成について説明します。

## アーキテクチャ詳細

### ネットワーク構成

```
┌─────────────────────────────────────────────────────────────────┐
│                        VPC (10.0.0.0/16)                         │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────┐    ┌─────────────────────┐            │
│  │   Public Subnet 1    │    │   Public Subnet 2    │           │
│  │   10.0.1.0/24       │    │   10.0.2.0/24       │           │
│  │   (NAT Gateway)     │    │   (NAT Gateway)     │           │
│  └─────────────────────┘    └─────────────────────┘           │
│                                                                  │
│  ┌─────────────────────┐    ┌─────────────────────┐           │
│  │  Private Subnet 1    │    │  Private Subnet 2    │          │
│  │   10.0.11.0/24      │    │   10.0.12.0/24      │          │
│  │   (ECS Tasks)       │    │   (ECS Tasks)       │          │
│  └─────────────────────┘    └─────────────────────┘          │
│                                                                 │
│  ┌─────────────────────┐    ┌─────────────────────┐          │
│  │ Database Subnet 1    │    │ Database Subnet 2    │         │
│  │   10.0.21.0/24      │    │   10.0.22.0/24      │         │
│  │   (Aurora RDS)      │    │   (Aurora RDS)      │         │
│  └─────────────────────┘    └─────────────────────┘         │
└──────────────────────────────────────────────────────────────┘
```

### コンポーネント詳細

#### 1. フロントエンド層

- **S3 静的ウェブサイトホスティング**
  - Kotlin/JS でビルドされた SPA
  - バージョニング有効
  - アクセスログ記録

- **CloudFront CDN**
  - グローバル配信
  - HTTPS 強制
  - キャッシュ最適化
  - WAF 統合（将来実装）

#### 2. アプリケーション層

- **Application Load Balancer (ALB)**
  - パブリックサブネットに配置
  - HTTPS リスナー
  - ヘルスチェック設定
  - ターゲットグループへのルーティング

- **ECS Fargate**
  - プライベートサブネットで実行
  - Rust バックエンド API
  - Auto Scaling 設定
  - CloudWatch Logs 統合

#### 3. データ層

- **Aurora PostgreSQL Serverless v2**
  - Multi-AZ 構成
  - 自動スケーリング (0.5-1.0 ACU)
  - 自動バックアップ (7日間)
  - Performance Insights 有効
  - 暗号化有効

#### 4. オーケストレーション層

- **Amazon MWAA (Managed Apache Airflow)**
  - Apache Airflow 2.8.1
  - プライベートサブネットで実行
  - 1-2 ワーカーの自動スケーリング
  - S3 からの DAG 読み込み

### セキュリティアーキテクチャ

#### ネットワークセキュリティ

1. **VPC 設計**
   - パブリック、プライベート、データベースサブネットの分離
   - NAT Gateway 経由のアウトバウンド通信

2. **セキュリティグループ**
   ```
   ALB SG: 
     - Ingress: 80, 443 from 0.0.0.0/0
     - Egress: All to ECS SG
   
   ECS SG:
     - Ingress: 8080 from ALB SG
     - Egress: 5432 to RDS SG, 443 to 0.0.0.0/0
   
   RDS SG:
     - Ingress: 5432 from ECS SG, MWAA SG
     - Egress: None
   
   MWAA SG:
     - Ingress: 443 from VPC CIDR
     - Egress: All
   ```

#### データ保護

1. **暗号化**
   - S3: SSE-S3
   - RDS: 保存時の暗号化
   - EBS: 暗号化有効
   - 転送中: TLS/SSL

2. **認証情報管理**
   - Secrets Manager: DB パスワード、API キー
   - Systems Manager Parameter Store: 設定値
   - IAM ロール: AWS サービス間の認証

### 高可用性設計

1. **Multi-AZ 構成**
   - すべてのコンポーネントが複数 AZ に分散
   - Aurora の自動フェイルオーバー
   - ALB によるヘルスチェックとトラフィック分散

2. **自動復旧**
   - ECS タスクの自動再起動
   - Aurora の自動バックアップとリストア
   - CloudWatch アラームによる通知

### スケーラビリティ

1. **水平スケーリング**
   - ECS: CPU/メモリベースの Auto Scaling
   - Aurora: ACU による自動スケーリング
   - MWAA: ワーカー数の動的調整

2. **垂直スケーリング**
   - ECS: タスク定義の更新で対応
   - Aurora: 最大 ACU の調整
   - MWAA: 環境クラスの変更

## コスト構造

### 開発環境のコスト見積もり（月額）

| サービス | 構成 | 推定コスト |
|---------|------|-----------|
| VPC | NAT Gateway x2 | $90 |
| RDS | Aurora Serverless v2 (0.5 ACU) | $60 |
| ECS | Fargate (0.25 vCPU, 0.5GB) | $20 |
| ALB | 基本料金 + トラフィック | $25 |
| S3 | ストレージ + リクエスト | $5 |
| CloudFront | トラフィック | $10 |
| MWAA | Small 環境 | $140 |
| **合計** | | **約 $350** |

### コスト最適化のポイント

1. **開発環境**
   - 夜間・週末の自動停止
   - 最小構成での運用
   - Spot インスタンスの活用（将来）

2. **本番環境**
   - Reserved Capacity の購入
   - Savings Plans の適用
   - 使用状況に応じたリソース調整

## 災害復旧計画

### バックアップ戦略

1. **データベース**
   - 自動バックアップ: 毎日
   - 保持期間: 7日間
   - ポイントインタイムリカバリ対応

2. **アプリケーション**
   - コンテナイメージ: ECR で管理
   - 設定: Terraform で管理
   - ソースコード: GitHub

### 復旧手順

1. **RTO (Recovery Time Objective): 4時間**
2. **RPO (Recovery Point Objective): 24時間**

復旧優先順位：
1. データベースの復元
2. アプリケーションの再デプロイ
3. フロントエンドの復旧
4. バッチ処理の再開

## 監視とアラート

### CloudWatch メトリクス

- ECS: CPU/メモリ使用率、タスク数
- RDS: 接続数、CPU 使用率、ストレージ容量
- ALB: レスポンスタイム、エラー率
- MWAA: DAG 実行状況、ワーカー使用率

### ログ管理

- アプリケーションログ: CloudWatch Logs
- アクセスログ: S3
- 監査ログ: CloudTrail

### アラート設定

- 高 CPU/メモリ使用率
- データベース接続エラー
- HTTP 5xx エラー率上昇
- DAG 実行失敗

## 今後の拡張計画

1. **セキュリティ強化**
   - AWS WAF の導入
   - AWS Shield Advanced
   - VPC Flow Logs

2. **パフォーマンス向上**
   - ElastiCache の導入
   - Lambda@Edge での処理
   - グローバル展開

3. **運用改善**
   - AWS X-Ray によるトレーシング
   - AWS Config による構成管理
   - AWS Systems Manager による運用自動化
# Airflow DAGs for NBA Trade Tracker

このディレクトリには、NBA Trade TrackerのApache Airflow DAGが含まれています。

## ドキュメント

詳細なドキュメントは[docs/airflow/](/docs/airflow/)を参照してください：

- [Apache Airflow DAGs](/docs/airflow/dags.md) - DAGの実装と運用方法
- [Amazon MWAA環境構築](/docs/airflow/mwaa-setup.md) - Terraformを使用したMWAA環境の構築方法

## ディレクトリ構造

```
airflow/
├── README.md          # このファイル
├── dags/              # DAGファイル
│   └── nba_rss_scraper.py
└── requirements.txt   # Python依存関係
```

## クイックスタート

```bash
# DAGのデプロイ
aws s3 cp dags/ s3://your-mwaa-bucket/dags/ --recursive

# requirements.txtのアップロード
aws s3 cp requirements.txt s3://your-mwaa-bucket/
```

詳細な手順は[ドキュメント](/docs/airflow/)を参照してください。
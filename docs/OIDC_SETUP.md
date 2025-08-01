# GitHub Actions OIDC プロバイダーセットアップ

## 概要
このプロジェクトでは、GitHub ActionsからAWSサービス（Amazon Translate）にアクセスするために、OIDCプロバイダーを使用しています。

## セットアップ手順

### 1. AWS側の設定

#### 1.1 OIDCプロバイダーの作成
```bash
aws iam create-open-id-connect-provider \
  --url https://token.actions.githubusercontent.com \
  --client-id-list sts.amazonaws.com \
  --thumbprint-list 6938fd4d98bab03faadb97b34396831e3780aea1
```

#### 1.2 IAMロールの作成
以下の信頼ポリシーを使用してIAMロールを作成します：

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "Federated": "arn:aws:iam::YOUR_ACCOUNT_ID:oidc-provider/token.actions.githubusercontent.com"
      },
      "Action": "sts:AssumeRoleWithWebIdentity",
      "Condition": {
        "StringEquals": {
          "token.actions.githubusercontent.com:aud": "sts.amazonaws.com"
        },
        "StringLike": {
          "token.actions.githubusercontent.com:sub": "repo:YOUR_GITHUB_ORG/YOUR_REPO_NAME:*"
        }
      }
    }
  ]
}
```

#### 1.3 必要な権限の付与
作成したロールに以下の権限を付与します：

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "translate:TranslateText",
        "translate:DetectDominantLanguage"
      ],
      "Resource": "*"
    }
  ]
}
```

### 2. GitHub側の設定

#### 2.1 Secretsの設定
GitHub リポジトリの Settings > Secrets and variables > Actions に以下を追加：

- `AWS_ROLE_ARN`: 作成したIAMロールのARN（例: `arn:aws:iam::123456789012:role/github-actions-iso-flow`）

### 3. 動作確認

1. プルリクエストを作成
2. GitHub Actionsが正常に実行され、AWS Translateへのアクセスが成功することを確認

## トラブルシューティング

### エラー: "Error: Could not assume role"
- IAMロールの信頼ポリシーが正しく設定されているか確認
- リポジトリ名が正しいか確認（大文字小文字に注意）

### エラー: "AccessDenied: User is not authorized to perform translate:TranslateText"
- IAMロールに適切な権限が付与されているか確認

## 参考資料
- [GitHub Actions での OpenID Connect の使用](https://docs.github.com/ja/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect)
- [AWS での GitHub Actions の OIDC 設定](https://aws.amazon.com/jp/blogs/security/use-iam-roles-to-connect-github-actions-to-actions-in-aws/)
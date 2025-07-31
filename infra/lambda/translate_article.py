import json
import boto3
import os
from typing import Dict, Any

translate_client = boto3.client('translate')

def lambda_handler(event: Dict[str, Any], context: Any) -> Dict[str, Any]:
    """
    記事を日本語に翻訳するLambda関数
    
    Input:
        {
            "articleId": "xxx",
            "title": "English title",
            "description": "English description"
        }
    
    Output:
        {
            "articleId": "xxx",
            "translatedTitle": "日本語タイトル",
            "translatedDescription": "日本語説明"
        }
    """
    try:
        article_id = event['articleId']
        title = event['title']
        description = event.get('description', '')
        
        # タイトルの翻訳
        title_response = translate_client.translate_text(
            Text=title,
            SourceLanguageCode='en',
            TargetLanguageCode='ja'
        )
        translated_title = title_response['TranslatedText']
        
        # 説明文の翻訳（存在する場合）
        translated_description = ''
        if description:
            desc_response = translate_client.translate_text(
                Text=description,
                SourceLanguageCode='en',
                TargetLanguageCode='ja'
            )
            translated_description = desc_response['TranslatedText']
        
        return {
            'articleId': article_id,
            'translatedTitle': translated_title,
            'translatedDescription': translated_description
        }
        
    except Exception as e:
        print(f"Error translating article {event.get('articleId', 'unknown')}: {str(e)}")
        raise
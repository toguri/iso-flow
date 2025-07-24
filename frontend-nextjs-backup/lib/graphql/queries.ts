import { gql } from '@apollo/client';

export const GET_TRADE_NEWS = gql`
  query GetTradeNews {
    tradeNews {
      id
      title
      description
      link
      source
      publishedAt
      category
    }
  }
`;

export const GET_NEWS_BY_CATEGORY = gql`
  query GetNewsByCategory($category: String!) {
    tradeNewsByCategory(category: $category) {
      id
      title
      description
      link
      source
      publishedAt
      category
    }
  }
`;

export const GET_NEWS_BY_SOURCE = gql`
  query GetNewsBySource($source: String!) {
    tradeNewsBySource(source: $source) {
      id
      title
      description
      link
      source
      publishedAt
      category
    }
  }
`;
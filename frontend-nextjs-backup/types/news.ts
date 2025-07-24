export interface NewsItem {
  id: string;
  title: string;
  description?: string;
  link: string;
  source: string;
  publishedAt: string;
  category: 'Trade' | 'Signing' | 'Other';
}

export interface NewsListProps {
  news: NewsItem[];
  loading?: boolean;
  error?: Error;
}
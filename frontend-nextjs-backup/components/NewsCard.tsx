'use client';

import { NewsItem } from '@/types/news';

interface NewsCardProps {
  news: NewsItem;
}

export default function NewsCard({ news }: NewsCardProps) {
  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleString('ja-JP', {
      year: 'numeric',
      month: 'numeric',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const getCategoryColor = (category: string) => {
    switch (category) {
      case 'Trade':
        return 'bg-red-100 text-red-800';
      case 'Signing':
        return 'bg-blue-100 text-blue-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  return (
    <div className="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow">
      <div className="flex justify-between items-start mb-2">
        <span className={`px-2 py-1 text-xs font-semibold rounded ${getCategoryColor(news.category)}`}>
          {news.category}
        </span>
        <span className="text-sm text-gray-500">{news.source}</span>
      </div>
      
      <h3 className="text-lg font-bold mb-2">
        <a 
          href={news.link} 
          target="_blank" 
          rel="noopener noreferrer"
          className="hover:text-nba-blue transition-colors"
        >
          {news.title}
        </a>
      </h3>
      
      {news.description && (
        <p className="text-gray-600 text-sm mb-3 line-clamp-2">
          {news.description}
        </p>
      )}
      
      <div className="text-xs text-gray-500">
        {formatDate(news.publishedAt)}
      </div>
    </div>
  );
}
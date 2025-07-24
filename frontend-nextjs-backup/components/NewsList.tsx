'use client';

import { NewsListProps } from '@/types/news';
import NewsCard from './NewsCard';

export default function NewsList({ news, loading, error }: NewsListProps) {
  if (loading) {
    return (
      <div className="flex justify-center items-center h-64">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-nba-blue"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
        エラーが発生しました: {error.message}
      </div>
    );
  }

  if (news.length === 0) {
    return (
      <div className="text-center text-gray-500 py-8">
        ニュースが見つかりませんでした
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {news.map((item) => (
        <NewsCard key={item.id} news={item} />
      ))}
    </div>
  );
}
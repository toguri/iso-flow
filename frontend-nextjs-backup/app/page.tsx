'use client';

import { useState, useEffect } from 'react';
import { useQuery } from '@apollo/client';
import { GET_TRADE_NEWS, GET_NEWS_BY_CATEGORY } from '@/lib/graphql/queries';
import NewsList from '@/components/NewsList';
import CategoryFilter from '@/components/CategoryFilter';
import { NewsItem } from '@/types/news';

export default function Home() {
  const [selectedCategory, setSelectedCategory] = useState('all');
  
  const { data, loading, error, refetch } = useQuery(
    selectedCategory === 'all' ? GET_TRADE_NEWS : GET_NEWS_BY_CATEGORY,
    {
      variables: selectedCategory !== 'all' ? { category: selectedCategory } : undefined,
      skip: false,
    }
  );

  const news: NewsItem[] = data?.tradeNews || data?.tradeNewsByCategory || [];

  return (
    <div>
      <div className="mb-8">
        <h2 className="text-3xl font-bold mb-4">最新ニュース</h2>
        <CategoryFilter 
          selected={selectedCategory} 
          onChange={setSelectedCategory}
        />
      </div>
      
      <NewsList news={news} loading={loading} error={error} />
      
      <div className="mt-8 text-center">
        <button
          onClick={() => refetch()}
          className="bg-nba-blue text-white px-6 py-2 rounded hover:bg-blue-700 transition-colors"
        >
          更新
        </button>
      </div>
    </div>
  );
}
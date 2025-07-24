'use client';

interface CategoryFilterProps {
  selected: string;
  onChange: (category: string) => void;
}

const categories = [
  { value: 'all', label: 'すべて' },
  { value: 'Trade', label: 'トレード' },
  { value: 'Signing', label: '契約' },
  { value: 'Other', label: 'その他' },
];

export default function CategoryFilter({ selected, onChange }: CategoryFilterProps) {
  return (
    <div className="flex gap-2 mb-6">
      {categories.map((category) => (
        <button
          key={category.value}
          onClick={() => onChange(category.value)}
          className={`px-4 py-2 rounded-full font-medium transition-colors ${
            selected === category.value
              ? 'bg-nba-blue text-white'
              : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
          }`}
        >
          {category.label}
        </button>
      ))}
    </div>
  );
}
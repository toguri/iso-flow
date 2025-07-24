import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'
import ApolloProviderWrapper from '@/components/ApolloProvider'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'NBA Trade Tracker',
  description: 'Track NBA trades, signings, and rumors in real-time',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="ja">
      <body className={inter.className}>
        <ApolloProviderWrapper>
          <nav className="bg-nba-blue text-white p-4">
            <div className="container mx-auto flex justify-between items-center">
              <h1 className="text-2xl font-bold">NBA Trade Tracker</h1>
              <div className="flex gap-4">
                <a href="/" className="hover:underline">Home</a>
                <a href="/trades" className="hover:underline">Trades</a>
                <a href="/signings" className="hover:underline">Signings</a>
              </div>
            </div>
          </nav>
          <main className="container mx-auto p-4">
            {children}
          </main>
        </ApolloProviderWrapper>
      </body>
    </html>
  )
}
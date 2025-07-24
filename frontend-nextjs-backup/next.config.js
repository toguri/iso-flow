/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  async rewrites() {
    return [
      {
        source: '/api/graphql',
        destination: 'http://localhost:8000/graphql',
      },
    ];
  },
};

module.exports = nextConfig;
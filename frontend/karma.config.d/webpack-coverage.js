// Webpackの設定にIstanbul loaderを追加
const path = require('path');
const webpackConfig = config.webpack || {};
webpackConfig.module = webpackConfig.module || {};
webpackConfig.module.rules = webpackConfig.module.rules || [];

// カバレッジ測定のためのistanbul-instrumenter-loaderを追加
webpackConfig.module.rules.push({
    test: /\.js$/,
    use: {
        loader: 'istanbul-instrumenter-loader',
        options: { esModules: true }
    },
    include: [
        path.resolve(__dirname, '../../../build/js/packages/iso-flow-frontend/kotlin')
    ],
    exclude: [
        /node_modules/,
        /test/
    ],
    enforce: 'post'
});

config.set({ webpack: webpackConfig });
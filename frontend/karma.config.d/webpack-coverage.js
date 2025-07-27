// Webpackの設定にIstanbul loaderを追加
const path = require('path');
const webpackConfig = config.webpack || {};
webpackConfig.module = webpackConfig.module || {};
webpackConfig.module.rules = webpackConfig.module.rules || [];

// ソースマップの設定を確実に有効化
webpackConfig.devtool = 'inline-source-map';

// カバレッジ測定のためのistanbul-instrumenter-loaderを追加
// フロントエンドのソースコードのみを対象にする
webpackConfig.module.rules.push({
    test: /\.js$/,
    use: {
        loader: 'istanbul-instrumenter-loader',
        options: { 
            esModules: true,
            produceSourceMap: true
        }
    },
    include: [
        // メインのソースコードのみを対象
        path.resolve(__dirname, '../../../build/js/packages/iso-flow-frontend/kotlin/iso-flow-frontend.js')
    ],
    exclude: [
        /node_modules/,
        /test/,
        /kotlin-kotlin-stdlib/,
        /kotlinx/,
        /common/
    ],
    enforce: 'post'
});

config.set({ webpack: webpackConfig });
// Webpackの設定にIstanbul loaderを追加
const path = require('path');
const webpackConfig = config.webpack || {};
webpackConfig.module = webpackConfig.module || {};
webpackConfig.module.rules = webpackConfig.module.rules || [];

// ソースマップの設定を確実に有効化
webpackConfig.devtool = 'inline-source-map';

// istanbul-instrumenter-loaderの設定
// Kotlin/JSで生成されたコードのうち、プロジェクトのコードを含む部分を対象にする
const projectRoot = path.resolve(__dirname, '../../..');
const frontendPackage = path.join(projectRoot, 'build/js/packages/iso-flow-frontend/kotlin');

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
        frontendPackage
    ],
    exclude: [
        /node_modules/,
        /webpack/,
        /test/
    ],
    enforce: 'post'
});

config.set({ webpack: webpackConfig });
// karma-coverageプラグインを登録
config.plugins = config.plugins || [];
config.plugins.push('karma-coverage');

// レポーターにcoverageを追加
config.reporters.push('coverage');

// カバレッジ対象ファイルの設定
// Webpackでバンドルされる前のファイルにカバレッジを適用
config.preprocessors = config.preprocessors || {};
config.preprocessors['kotlin/**/*.js'] = ['sourcemap', 'coverage'];
config.preprocessors['kotlin/iso-flow-frontend.js'] = ['sourcemap', 'coverage'];

// カバレッジレポートの設定
config.coverageReporter = {
    reporters: [
        { type: 'lcov', subdir: '.' },
        { type: 'json', subdir: '.' },
        { type: 'text-summary' }
    ],
    dir: 'build/reports/coverage/',
    // ソースファイルのパスを修正
    fixWebpackSourcePaths: true,
    skipFilesWithNoCoverage: true,
    // カバレッジ対象を制限
    check: {
        global: {
            excludes: [
                '**/kotlin-kotlin-stdlib/**',
                '**/kotlinx/**',
                '**/common/**',
                '**/test/**'
            ]
        }
    }
};
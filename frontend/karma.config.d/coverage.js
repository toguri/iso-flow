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
    type: 'lcov',
    dir: 'build/reports/coverage/',
    subdir: '.',
    file: 'lcov.info'
};
// karma-coverageプラグインを登録
config.plugins = config.plugins || [];
config.plugins.push('karma-coverage');

// レポーターにcoverageを追加
config.reporters.push('coverage');

// カバレッジ対象ファイルの設定
config.preprocessors = config.preprocessors || {};
// 既存のpreprocessorsを保持しつつ、カバレッジを追加しない
// （Webpackでバンドルされたコードには適用しない）

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
    includeAllSources: false,  // 参照されていないファイルは含まない
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
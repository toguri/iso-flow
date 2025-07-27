// karma-coverageプラグインを登録
config.plugins = config.plugins || [];
config.plugins.push('karma-coverage');

// レポーターにcoverageを追加
config.reporters.push('coverage');

// カバレッジ対象ファイルの設定
config.preprocessors = config.preprocessors || {};
// Webpack処理後のファイルにカバレッジを適用
const frontendSource = 'kotlin/iso-flow-frontend.js';
if (!config.preprocessors[frontendSource]) {
    config.preprocessors[frontendSource] = [];
}
config.preprocessors[frontendSource].push('coverage');

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
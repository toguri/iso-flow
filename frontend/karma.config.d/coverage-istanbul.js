// karma-coverage-istanbul-reporterの設定
config.plugins = config.plugins || [];
config.plugins.push('karma-coverage-istanbul-reporter');

// レポーターを置き換え
const coverageIndex = config.reporters.indexOf('coverage');
if (coverageIndex > -1) {
    config.reporters.splice(coverageIndex, 1);
}
config.reporters.push('coverage-istanbul');

// カバレッジレポーターの設定
config.coverageIstanbulReporter = {
    reports: ['lcovonly', 'text-summary'],
    dir: require('path').join(__dirname, '../build/reports/coverage'),
    combineBrowserReports: true,
    skipFilesWithNoCoverage: false,
    verbose: true,
    fixWebpackSourcePaths: true,
    
    // 重要: thresholdsを設定してファイルを含める
    thresholds: {
        emitWarning: false,
        global: {
            statements: 0,
            lines: 0,
            branches: 0,
            functions: 0
        }
    }
};
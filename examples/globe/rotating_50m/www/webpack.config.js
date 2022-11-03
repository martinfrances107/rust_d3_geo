const CopyWebpackPlugin = require('copy-webpack-plugin');
const ESLintPlugin = require('eslint-webpack-plugin');

const path = require('path')

module.exports = {
    entry: './bootstrap.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'bootstrap.js'
    },
    mode: 'production',
    experiments: { syncWebAssembly: true },
    plugins: [
        new ESLintPlugin(),
        new CopyWebpackPlugin({
            patterns: [
                { from: 'index.html' }
            ]
        })
    ]
}

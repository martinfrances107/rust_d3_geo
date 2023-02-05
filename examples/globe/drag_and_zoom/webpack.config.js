const CopyWebpackPlugin = require('copy-webpack-plugin')
const ESLintPlugin = require('eslint-webpack-plugin')
const path = require('path')
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: './js/index.ts',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.js'
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  performance: {
    maxEntrypointSize: 1 * 1024 * 1024,
    maxAssetSize: 1 * 1024 * 1024
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js']
  },
  mode: 'development',
  devtool: 'inline-source-map',
  experiments: { syncWebAssembly: true },
  plugins: [
    // new ESLintPlugin(),
    new CopyWebpackPlugin({
      patterns: [
        { from: 'index.html' },
        { from: 'public/world-atlas', to: 'world-atlas' }
      ]
    }),
    new WasmPackPlugin({
      crateDirectory: __dirname,
      args: '--log-level warn',
      extraArgs: '',
      forceMode: 'development'
    }),

  ]
}

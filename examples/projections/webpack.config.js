const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.ts"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  module: {
    rules: [
      {
        test: /\.ts?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  devServer: {
    static: {
      directory: dist,
    }
  },
  performance: {
    // HACK: the .wasm file is too big it should be chunked.
    // but I think atm I need to use asyncWebAssembley for that
    // TODO must resolve.
    maxAssetSize: 3 * 1024 * 1024
  },
  experiments: { syncWebAssembly: true, },
  plugins: [
    new CopyPlugin({
      patterns: [
        path.resolve(__dirname, "static")
      ]
    }),

    new WasmPackPlugin({
      crateDirectory: __dirname,
      args: '--log-level warn',
      extraArgs: '',
      // forceMode: 'development'
    }),
  ]
};

const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
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
    maxAssetSize: 512000
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
      forceMode: 'development'
    }),
  ]
};

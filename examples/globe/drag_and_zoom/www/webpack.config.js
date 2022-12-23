const CopyWebpackPlugin = require( 'copy-webpack-plugin' )
const ESLintPlugin = require( 'eslint-webpack-plugin' )

const path = require( 'path' )

module.exports = {
  entry: './bootstrap.ts',
  output: {
    path: path.resolve( __dirname, 'dist' ),
    filename: 'bootstrap.js'
  },
  module: {
    rules: [
      {
        test: /\.(ts|js)?$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: ['@babel/preset-env', '@babel/preset-typescript']
          }
        }
      }
    ]
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js']
  },
  performance: {
    maxEntrypointSize: 1 * 1024 * 1024,
    maxAssetSize: 1 * 1024 * 1024
  },
  mode: 'development',
  devtool: 'inline-source-map',
  experiments: { syncWebAssembly: true },
  plugins: [
    new ESLintPlugin(),
    new CopyWebpackPlugin( {
      patterns: [
        { from: 'index.html' },
        { from: 'index2.html' },
        { from: 'index2c.html' },
        { from: 'public/world-atlas', to: 'world-atlas' }
      ]
    } )
  ]
}

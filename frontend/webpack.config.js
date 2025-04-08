const path = require('path');
const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = {
  mode: "development",
  entry: "./bundle.js",
  experiments: {
    asyncWebAssembly: true,
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bundle.js",
  },
  devServer: {
    static: path.join(__dirname, "dist"),
    hot: true,
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from:"index.html", to:"index.html" },
        { from:"pkg", to: "pkg" },
      ]
    })
  ],
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: 'webassembly/async',
      },
    ],
  },
};

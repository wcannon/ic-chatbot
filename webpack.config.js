const path = require("path");
const webpack = require("webpack");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

let localCanisters, prodCanisters, canisters;

function initCanisterIds() {
  try {
    localCanisters = require(path.resolve(".dfx", "local", "canister_ids.json"));
  } catch (error) {
    console.log("No local canister_ids.json found. Continuing production");
  }
  try {
    prodCanisters = require(path.resolve("canister_ids.json"));
  } catch (error) {
    console.log("No production canister_ids.json found. Continuing with local");
  }

  const network =
    process.env.DFX_NETWORK ||
    (process.env.NODE_ENV === "production" ? "ic" : "local");

  canisters = network === "local" ? localCanisters : prodCanisters;

  for (const canister in canisters) {
    process.env[canister.toUpperCase() + "_CANISTER_ID"] =
      canisters[canister][network];
  }
}
initCanisterIds();

const isDevelopment = process.env.NODE_ENV !== "production";
const asset_entry = path.join(
  "src",
<<<<<<< HEAD
  "ic_chatbot_assets",
=======
  "rust_profile_assets",
>>>>>>> bb7457b (initital commit)
  "src",
  "index.html"
);

module.exports = {
  target: "web",
  mode: isDevelopment ? "development" : "production",
  entry: {
    // The frontend.entrypoint points to the HTML file for this build, so we need
    // to replace the extension to `.js`.
<<<<<<< HEAD
    index: path.join(__dirname, asset_entry).replace(/\.html$/, ".jsx"),
=======
    index: path.join(__dirname, asset_entry).replace(/\.html$/, ".js"),
>>>>>>> bb7457b (initital commit)
  },
  devtool: isDevelopment ? "source-map" : false,
  optimization: {
    minimize: !isDevelopment,
    minimizer: [new TerserPlugin()],
  },
  resolve: {
    extensions: [".js", ".ts", ".jsx", ".tsx"],
    fallback: {
      assert: require.resolve("assert/"),
      buffer: require.resolve("buffer/"),
      events: require.resolve("events/"),
      stream: require.resolve("stream-browserify/"),
      util: require.resolve("util/"),
    },
  },
  output: {
    filename: "index.js",
<<<<<<< HEAD
    path: path.join(__dirname, "dist", "ic_chatbot_assets"),
=======
    path: path.join(__dirname, "dist", "rust_profile_assets"),
>>>>>>> bb7457b (initital commit)
  },

  // Depending in the language or framework you are using for
  // front-end development, add module loaders to the default
  // webpack configuration. For example, if you are using React
  // modules and CSS as described in the "Adding a stylesheet"
  // tutorial, uncomment the following lines:
<<<<<<< HEAD
  module: {
    rules: [
      { test: /\.(ts|tsx|jsx)$/, loader: "ts-loader" },
      { test: /\.css$/, use: ['style-loader','css-loader'] }
    ]
  },
=======
  // module: {
  //  rules: [
  //    { test: /\.(ts|tsx|jsx)$/, loader: "ts-loader" },
  //    { test: /\.css$/, use: ['style-loader','css-loader'] }
  //  ]
  // },
>>>>>>> bb7457b (initital commit)
  plugins: [
    new HtmlWebpackPlugin({
      template: path.join(__dirname, asset_entry),
      cache: false
    }),
    new CopyPlugin({
      patterns: [
        {
<<<<<<< HEAD
          from: path.join(__dirname, "src", "ic_chatbot_assets", "assets"),
          to: path.join(__dirname, "dist", "ic_chatbot_assets"),
=======
          from: path.join(__dirname, "src", "rust_profile_assets", "assets"),
          to: path.join(__dirname, "dist", "rust_profile_assets"),
>>>>>>> bb7457b (initital commit)
        },
      ],
    }),
    new webpack.EnvironmentPlugin({
      NODE_ENV: 'development',
<<<<<<< HEAD
      IC_CHATBOT_CANISTER_ID: canisters["ic_chatbot"]
=======
      RUST_PROFILE_CANISTER_ID: canisters["rust_profile"]
>>>>>>> bb7457b (initital commit)
    }),
    new webpack.ProvidePlugin({
      Buffer: [require.resolve("buffer/"), "Buffer"],
      process: require.resolve("process/browser"),
    }),
  ],
  // proxy /api to port 8000 during development
  devServer: {
    proxy: {
      "/api": {
        target: "http://localhost:8000",
        changeOrigin: true,
        pathRewrite: {
          "^/api": "/api",
        },
      },
    },
    hot: true,
<<<<<<< HEAD
    contentBase: path.resolve(__dirname, "./src/ic_chatbot_assets"),
=======
    contentBase: path.resolve(__dirname, "./src/rust_profile_assets"),
>>>>>>> bb7457b (initital commit)
    watchContentBase: true
  },
};

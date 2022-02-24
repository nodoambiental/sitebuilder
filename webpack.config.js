// Generated using webpack-cli https://github.com/webpack/webpack-cli

const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const EventHooksPlugin = require("event-hooks-webpack-plugin");
const fs = require("fs-extra");

const isProduction = process.env.NODE_ENV == "production";

const config = {
    target: "web",
    entry: "./src/bundle.ts",
    output: {
        path: path.resolve(__dirname, "build/webpack"),
        clean: true,
        filename: "bundle.js",
    },
    plugins: [
        new EventHooksPlugin({
            afterEmit: (compilation, done) => {
                console.log(
                    "Copying compiled bundle to the zola static files directory"
                );
                fs.copy(
                    "build/webpack/bundle.js",
                    "build/webpack/static/bundle.js",
                    done
                );
            },
        }),
        new HtmlWebpackPlugin({
            template: "src/templates/index.pug",
            filename: "templates/index.html",
            chunks: ["index"],
        }),

        new CopyWebpackPlugin({
            patterns: [
                { from: "src/static", to: "static/" },
                { from: "src/config.toml", to: "config.toml" },
                { from: "src/theme.toml", to: "theme.toml" },
                //{ from: "build/webpack/main.js", to: "static/main.js" },
                //{ from: "src/content/", to: "content/" },
            ],
        }),
    ],
    module: {
        rules: [
            {
                test: /\.(ts|tsx)$/i,
                loader: "ts-loader",
                exclude: ["/node_modules/"],
            },
            {
                test: /\.s[ac]ss$/i,
                use: [
                    "style-loader",
                    "css-loader",
                    "postcss-loader",
                    "sass-loader",
                ],
            },
            {
                test: /\.pug$/,
                loader: "pug-loader",
            },
            {
                test: /\.(eot|svg|ttf|woff|woff2|png|jpg|gif)$/i,
                type: "asset",
            },

            // Add your rules for custom modules here
            // Learn more about loaders from https://webpack.js.org/loaders/
        ],
    },
    resolve: {
        extensions: [".tsx", ".ts", ".js"],
    },
};

module.exports = () => {
    if (isProduction) {
        config.mode = "production";
    } else {
        config.mode = "development";
    }
    return config;
};

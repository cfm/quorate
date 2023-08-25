const NodePolyfillPlugin = require('node-polyfill-webpack-plugin');

process.env.VUE_APP_URL =
  process.env.CONTEXT == 'production'
    ? process.env.URL
    : process.env.DEPLOY_PRIME_URL;

module.exports = {
  chainWebpack: (config) => {
    config.plugin('polyfills').use(NodePolyfillPlugin);
  },
  transpileDependencies: ['vuetify'],
};

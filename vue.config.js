const NodePolyfillPlugin = require('node-polyfill-webpack-plugin');

process.env.VUE_APP_URL =
  process.env.CONTEXT == 'production'
    ? process.env.URL
    : process.env.DEPLOY_PRIME_URL;

module.exports = {
  chainWebpack: (config) => {
    config.plugin('polyfills').use(NodePolyfillPlugin);
  },
  configureWebpack: {
    resolve: {
      alias: {
        // For @apis/proxy-solver → datauri → fs/promises, per
        // <https://github.com/readmeio/api/issues/604#issuecomment-1533950156>.
        fs: 'memfs',
      },
    },
  },
  transpileDependencies: ['vuetify'],
};

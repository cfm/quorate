process.env.VUE_APP_URL =
  process.env.CONTEXT == 'production'
    ? process.env.URL
    : process.env.DEPLOY_PRIME_URL;

module.exports = {
  transpileDependencies: ['vuetify'],
};

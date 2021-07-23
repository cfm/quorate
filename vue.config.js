process.env.VUE_APP_URL = process.env.DEPLOY_PRIME_URL || process.env.URL;

module.exports = {
  transpileDependencies: ['vuetify'],
};

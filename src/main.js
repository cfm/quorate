import Rollbar from 'rollbar';
import Vue from 'vue';
import App from './App.vue';
import vuetify from './plugins/vuetify';
import store from './store';

import VueTimeago from 'vue-timeago';
import VueTimers from 'vue-timers';

Vue.prototype.$rollbar = new Rollbar({
  accessToken: process.env.VUE_APP_ROLLBAR_CLIENT_TOKEN,
  captureUncaught: true,
  captureUnhandledRejections: true,
  payload: {
    environment: process.env.VUE_APP_NETLIFY_SITE_NAME,
  },
});
Vue.config.errorHandler = (err, vm) => {
  vm.$rollbar.error(err);
  throw err; // rethrow
};

Vue.config.productionTip = false;

Vue.use(VueTimeago, {
  locale: 'en',
});
Vue.use(VueTimers);

new Vue({
  vuetify,
  store,
  render: (h) => h(App),
}).$mount('#app');

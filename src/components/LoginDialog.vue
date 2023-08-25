<template>
  <v-dialog value="true" persistent max-width="50%">
    <v-card>
      <v-card-title>Please log in via Salesforce.</v-card-title>
      <v-card-actions>
        <v-btn @click="login()" :title="loginUrl">Log In</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script>
import jsforce from 'jsforce';

export default {
  name: 'LoginDialog',

  computed: {
    loginUrl() {
      return process.env.VUE_APP_SALESFORCE_LOGIN_URL;
    },
  },

  methods: {
    login() {
      jsforce.browser.login();
    },
  },
};
</script>

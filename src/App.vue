<template>
  <v-app>
    <v-main>
      <Login v-if="loginRequired" />
      <Loading v-if="operationIsInProgress" />
      <v-snackbar v-model="showConnectedToSalesforce" color="success">
        Connected to Salesforce
      </v-snackbar>
      <v-snackbar v-model="showError" color="error">
        {{ lastError }}
      </v-snackbar>
      <v-app-bar>
        <v-chip @click="sync()">
          <v-avatar><v-icon>mdi-table-refresh</v-icon></v-avatar>
          <template v-if="lastSync">
            Roster updated&nbsp;
            <timeago :datetime="lastSync" :auto-update="60" />
          </template>
          <template v-else> Refresh </template>
        </v-chip>
        <v-chip :ripple="false">
          <v-avatar>{{ members.length }}</v-avatar> members
        </v-chip>
        <v-chip :ripple="false">
          <v-avatar>{{ present.length }}</v-avatar> present
        </v-chip>
      </v-app-bar>
      <Wizard />
    </v-main>
  </v-app>
</template>

<script>
import jsforce from 'jsforce';
import { mapMutations, mapState } from 'vuex';

import { FIELDS } from '@/constants';
import Loading from './components/Loading';
import Login from './components/Login';
import Wizard from './components/Wizard';

export default {
  name: 'App',

  components: {
    Loading,
    Login,
    Wizard,
  },

  data() {
    return {
      conn: null,
      lastError: null,
      lastSync: null,

      showConnectedToSalesforce: false,
      showError: false,
    };
  },

  computed: {
    ...mapState({
      members: (state) => state.memberList,
      present: (state) => state.presentList,

      operationIsInProgress: (state) => state.operationIsInProgress,
      operationHadError: (state) => state.operationHadError,
    }),

    loginRequired: function () {
      return this.conn == null;
    },
  },

  watch: {
    conn: function (conn) {
      if (conn) this.showConnectedToSalesforce = true;
    },
    operationHadError(error) {
      this.lastError = error;
    },
    lastError(error) {
      this.showError = error != null;
    },
  },

  async mounted() {
    jsforce.browser.init({
      clientId: process.env.VUE_APP_SALESFORCE_CLIENT_ID,
      loginUrl: process.env.VUE_APP_SALESFORCE_LOGIN_URL,
      redirectUri:
        process.env.VUE_APP_SALESFORCE_REDIRECT_URI || process.env.VUE_APP_URL,
    });
    jsforce.browser.on('connect', (conn) => {
      console.log('Connected to Salesforce');
      this.conn = conn;
      this.sync();
    });
  },

  methods: {
    ...mapMutations([
      'replaceMemberList',
      'startOperation',
      'saveOperationError',
      'finishOperation',
    ]),
    sync() {
      this.startOperation();
      this.conn.query(
        `SELECT ${FIELDS.join(
          ', ',
        )} FROM Contact WHERE Is_Current_TA_Member__c = true`,
        (err, res) => {
          this.finishOperation();
          if (err) {
            this.saveOperationError(err);
            return;
          }
          this.lastSync = Date.now();
          this.replaceMemberList(res.records);
        },
      );
    },
  },
};
</script>

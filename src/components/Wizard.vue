<template>
  <v-stepper>
    <v-stepper-header>
      <v-stepper-step step="1" :editable="haveMemberList">
        Take attendance
      </v-stepper-step>
      <v-stepper-step step="2" :editable="haveAttendance">
        <v-badge dot :color="solverApiStatusColor"> Assign proxies </v-badge>
      </v-stepper-step>
      <v-stepper-step step="3" :editable="haveAttendance">
        Summary and transcript
      </v-stepper-step>
    </v-stepper-header>
    <v-stepper-items>
      <v-stepper-content step="1">
        <TakeAttendance />
      </v-stepper-content>
      <v-stepper-content step="2">
        <AssignProxies />
      </v-stepper-content>
      <v-stepper-content step="3">
        <Readout />
      </v-stepper-content>
    </v-stepper-items>
  </v-stepper>
</template>

<script>
import { mapGetters, mapMutations, mapState } from 'vuex';

import AssignProxies from './AssignProxies';
import Readout from './Readout';
import TakeAttendance from './TakeAttendance';

export default {
  name: 'Wizard',

  components: {
    AssignProxies,
    Readout,
    TakeAttendance,
  },

  data() {
    return {
      solverApiIsAvailable: false,
    };
  },

  computed: {
    ...mapState({
      members: (state) => state.memberList,
      present: (state) => state.presentList,
    }),
    ...mapGetters(['total', 'present']),

    haveAttendance() {
      return this.present > 0;
    },
    haveMemberList() {
      return this.total > 0;
    },

    solverApiStatusColor() {
      return this.solverApiIsAvailable ? 'green' : 'orange';
    },
  },

  async mounted() {
    await this.checkSolverApi();
  },

  methods: {
    ...mapMutations(['saveOperationError']),
    async checkSolverApi() {
      try {
        let res = await fetch(
          `${process.env.VUE_APP_PROXY_SOLVER_API}/health/ready`,
        );
        this.solverApiIsAvailable = res.status == 204;
      } catch (err) {
        this.saveOperationError(err);
      }
    },
  },

  timers: {
    checkSolverApi: {
      time: 60 * 1000,
      autostart: true,
      repeat: true,
    },
  },
};
</script>

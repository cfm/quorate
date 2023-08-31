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
        <TakeAttendanceStep />
      </v-stepper-content>
      <v-stepper-content step="2">
        <AssignProxiesStep />
      </v-stepper-content>
      <v-stepper-content step="3">
        <ReadoutStep />
      </v-stepper-content>
    </v-stepper-items>
  </v-stepper>
</template>

<script>
import { mapGetters, mapMutations, mapState } from 'vuex';

import solver from '../solver';

import AssignProxiesStep from './AssignProxiesStep';
import ReadoutStep from './ReadoutStep';
import TakeAttendanceStep from './TakeAttendanceStep';

export default {
  name: 'AttendanceWizard',

  components: {
    AssignProxiesStep,
    ReadoutStep,
    TakeAttendanceStep,
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

  methods: {
    ...mapMutations(['saveOperationError']),
    async checkSolverApi() {
      try {
        let res = await solver.get_health_ready();
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
      immediate: true,
      repeat: true,
    },
  },
};
</script>

<template>
  <v-container>
    <dl>
      <dt>Attendance taken</dt>
      <dd>{{ attendanceWasTaken }}</dd>
      <dt>Proxies assigned</dt>
      <dd>{{ proxiesWereAssigned }}</dd>
      <dt><h2>Members</h2></dt>
      <dd>
        <dl>
          <dt>Total</dt>
          <dd>{{ total }}</dd>
          <dt>Present</dt>
          <dd>{{ present }}</dd>
          <dt>Represented</dt>
          <dd>{{ represented }}</dd>
        </dl>
      </dd>
      <!--<dt><h2>Quorum requirements</h2></dt>-->
      <dt><h2>Transcript</h2></dt>
      <dd>
        <ol>
          <li v-for="member in roster" :key="member.Id">
            {{ member.FirstName }} {{ member.LastName }}
            <template v-if="presentList.includes(member.Id)">
              is present
            </template>
            <template v-else-if="representedList.includes(member.Id)">
              is absent and represented by proxy
            </template>
            <template v-else>is absent and not represented </template>
          </li>
        </ol>
      </dd>
    </dl>
  </v-container>
</template>

<script>
import { mapGetters, mapState } from 'vuex';

export default {
  name: 'Readout',

  computed: {
    ...mapState({
      presentList: (state) => state.presentList,
      representedList: (state) => state.representedList,
    }),
    ...mapGetters([
      'roster',
      'total',
      'present',
      'represented',
      'attendanceWasTaken',
      'proxiesWereAssigned',
    ]),
  },
};
</script>

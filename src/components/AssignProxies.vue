<template>
  <v-container>
    <v-btn @click="doAssignProxies">Assign Proxies</v-btn>
    <template v-if="represented > 0">
      <h2>Assignments as of {{ proxiesWereAssigned }}</h2>
      <ul>
        <li
          v-for="[holder, assignments] in Object.entries(assignments)"
          :key="holder"
        >
          {{ getMemberById(holder).FirstName }}
          {{ getMemberById(holder).LastName }} holds
          {{ assignments.length }} proxy/ies for:
          <span v-for="(assignment, key) in assignments" :key="assignment">
            {{ getMemberById(assignment).FirstName }}
            {{ getMemberById(assignment).LastName
            }}<span v-if="key + 1 != assignments.length">,</span>
          </span>
        </li>
      </ul>
    </template>
  </v-container>
</template>

<script>
import { mapGetters, mapMutations, mapState } from 'vuex';

import { fromPairs } from 'lodash';

export default {
  name: 'AssignProxies',

  data: () => ({
    transcript: '',
    proxies: null,
  }),

  computed: {
    ...mapState({
      memberList: (state) => state.memberList,
      presentList: (state) => state.presentList,
    }),
    ...mapGetters([
      'getMemberById',
      'roster',
      'represented',
      'proxiesWereAssigned',
    ]),
    assignments: function () {
      return fromPairs(
        this.roster
          .map((member) => member.Id)
          .map((id) => {
            if (id in this._assignments) return [id, this._assignments[id]];
          })
          .filter((x) => x != undefined),
      );
    },
    _assignments: function () {
      let assignments = {};
      if (!this.proxies) return assignments;

      Object.entries(this.proxies).forEach(([represented, holder]) => {
        represented = represented.trim();
        holder = holder.trim();

        if (assignments[holder]) {
          assignments[holder].push(represented);
        } else {
          assignments[holder] = [represented];
        }
      });

      return assignments;
    },
  },

  methods: {
    ...mapMutations([
      'replaceRepresentedList',
      'startOperation',
      'saveOperationError',
      'finishOperation',
    ]),
    async doAssignProxies() {
      try {
        this.startOperation();
        let res = await fetch(
          `${process.env.VUE_APP_PROXY_SOLVER_API}/solve/`,
          {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify({
              memberList: this.memberList,
              presentList: this.presentList,
            }),
          },
        );
        this.proxies = await res.json();
        this.replaceRepresentedList(Object.keys(this.proxies));
      } catch (err) {
        this.saveOperationError(err);
      } finally {
        this.finishOperation();
      }
    },
  },
};
</script>

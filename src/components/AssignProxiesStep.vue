<template>
  <v-container>
    <v-btn @click="doAssignProxiesStep">Assign Proxies</v-btn>
    <template v-if="represented > 0">
      <h2>Assignments as of {{ proxiesWereAssigned }}</h2>
      <ul>
        <li
          v-for="[holder, representing] in Object.entries(assignments)"
          :key="holder"
        >
          {{ getMemberById(holder).FirstName }}
          {{ getMemberById(holder).LastName }} holds
          {{ assignments.length }} proxy/ies for:
          <span v-for="(represented, key) in representing" :key="represented">
            {{ getMemberById(represented).FirstName }}
            {{ getMemberById(represented).LastName
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

import { MAX_REPRESENTATION } from '@/constants';

export default {
  name: 'AssignProxiesStep',

  data: () => ({
    transcript: '',
  }),

  computed: {
    ...mapState({
      memberList: (state) => state.memberList,
      presentList: (state) => state.presentList,
      representation: (state) => state.representation,
    }),
    ...mapGetters([
      'getMemberById',
      'getProxyIdListForMemberById',
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
      if (!this.representation) return assignments;

      Object.entries(this.representation).forEach(([represented, holder]) => {
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
      'updateRepresentation',
      'startOperation',
      'saveOperationError',
      'finishOperation',
    ]),
    async doAssignProxiesStep() {
      try {
        this.startOperation();
        let res = await fetch(
          `${process.env.VUE_APP_PROXY_SOLVER_API}/solution/${MAX_REPRESENTATION}`,
          {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify({
              members: this.memberList.map((member) => {
                return {
                  id: member.Id,
                  preferences: this.getProxyIdListForMemberById(member.Id),
                };
              }),
              members_present: this.presentList,
            }),
          },
        );
        const solution = await res.json();
        this.updateRepresentation(solution.represented);
      } catch (err) {
        this.saveOperationError(err);
        throw err;
      } finally {
        this.finishOperation();
      }
    },
  },
};
</script>

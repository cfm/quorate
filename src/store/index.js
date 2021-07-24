import Vue from 'vue';
import Vuex from 'vuex';

import moment from 'moment';
import { sortBy } from 'lodash';

import { PROXY_FIELDS } from '@/constants';
import * as rules from '@/rules';

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    memberList: [],
    presentList: [],
    representedList: [],

    operationIsInProgress: false,
    operationHadError: undefined,

    _attendanceWasTakenTimestamp: undefined,
    _proxiesWereAssignedTimestamp: undefined,
  },
  mutations: {
    replaceMemberList(state, memberList) {
      state.memberList = memberList;
    },
    replacePresentList(state, presentList) {
      state.presentList = presentList;
      state._attendanceWasTakenTimestamp = Date.now();
    },
    replaceRepresentedList(state, representedList) {
      state.representedList = representedList;
      state._proxiesWereAssignedTimestamp = Date.now();
    },
    startOperation(state) {
      state.operationIsInProgress = true;
    },
    saveOperationError(state, error) {
      state.operationHadError = error;
    },
    finishOperation(state) {
      state.operationIsInProgress = false;
    },
  },
  getters: {
    getMemberById: (state) => (id) => {
      return state.memberList.find((member) => member.Id == id);
    },
    getMemberLastNameById: (state, getters) => (id) => {
      const member = getters.getMemberById(id);
      if (member != undefined) return member.LastName;
      return '';
    },
    getProxyLastNamesForMemberById: (state, getters) => (id) => {
      const member = getters.getMemberById(id);
      const proxies = {};
      PROXY_FIELDS.forEach((k) => {
        const proxy = getters.getMemberLastNameById(member[k]);
        proxies[k] = proxy;
      });
      return proxies;
    },

    roster: (state) => sortBy(state.memberList, ['LastName', 'FirstName']),
    total: (state) => state.memberList.length,
    present: (state) => state.presentList.length,
    represented: (state) => state.representedList.length,

    membersQuorumPresentThreshold: (state, getters) =>
      rules.membersQuorumPresentThreshold(
        getters.present,
        getters.represented,
        getters.total,
      ),
    membersQuorumPresentOrRepresentedThreshold: (state, getters) =>
      rules.membersQuorumPresentOrRepresentedThreshold(
        getters.present,
        getters.represented,
        getters.total,
      ),
    haveMembersQuorum: (state, getters) =>
      getters.total > 0 &&
      getters.present >= getters.membersQuorumPresentThreshold &&
      getters.present + getters.represented >=
        getters.membersQuorumPresentOrRepresentedThreshold,

    membershipElectionThreshold: (state, getters) =>
      rules.membershipElectionThreshold(
        getters.present,
        getters.represented,
        getters.total,
      ),
    canElectMembers: (state, getters) =>
      getters.total > 0 &&
      getters.present + getters.represented >= getters.bylawsAmendmentThreshold,

    bylawsAmendmentThreshold: (state, getters) =>
      rules.bylawsAmendmentThreshold(
        getters.present,
        getters.represented,
        getters.total,
      ),
    canPassBylawsAmendment: (state, getters) =>
      getters.total > 0 &&
      getters.present + getters.represented >= getters.bylawsAmendmentThreshold,

    constitutionalAmendmentThreshold: (state, getters) =>
      rules.constitutionalAmendmentThreshold(
        getters.present,
        getters.represented,
        getters.total,
      ),
    canPassConstitutionalAmendment: (state, getters) =>
      getters.total > 0 &&
      getters.present + getters.represented >=
        getters.constitutionalAmendmentThreshold,

    directorsQuorumThreshold: (state, getters) =>
      rules.directorsQuorumThreshold(
        getters.present,
        getters.represented,
        getters.total,
      ),
    haveDirectorsQuorum: (state, getters) =>
      getters.total > 0 && getters.present >= getters.directorsQuorumThreshold,

    attendanceWasTaken: (state) => {
      const ts = state._attendanceWasTakenTimestamp;
      if (ts) return moment(ts).toString();
      return 'never';
    },
    proxiesWereAssigned: (state) => {
      const ts = state._proxiesWereAssignedTimestamp;
      if (ts) return moment(ts).toString();
      return 'never';
    },
  },
  modules: {},
});

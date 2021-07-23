import Vue from 'vue';
import Vuex from 'vuex';

import moment from 'moment';
import { sortBy } from 'lodash';

import { PROXY_FIELDS } from '@/constants';

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

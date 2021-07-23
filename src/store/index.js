import Vue from 'vue';
import Vuex from 'vuex';

import vuexLocal from '@/plugins/vuex-persist';

import { PROXY_FIELDS } from '@/constants';

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    memberList: [],
    presentList: [],
    operationIsInProgress: false,
    operationHadError: undefined,
  },
  mutations: {
    replaceMemberList(state, memberList) {
      state.memberList = memberList;
    },
    replacePresentList(state, presentList) {
      state.presentList = presentList;
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
  },
  modules: {},

  plugins: [vuexLocal.plugin],
});

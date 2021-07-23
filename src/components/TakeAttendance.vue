<template>
  <v-container>
    <v-text-field v-model="search" placeholder="search roster" clearable />
    <v-data-table
      no-data-text="The roster of members is empty."
      v-model="present"
      dense
      disable-pagination
      :headers="headers"
      hide-default-footer
      :items="members"
      item-key="Id"
      show-select
      fixed-header
      must-sort
      sort-by="LastName"
      :search="search"
    >
    </v-data-table>
  </v-container>
</template>

<script>
import { mapMutations, mapState } from 'vuex';

export default {
  name: 'TakeAttendance',

  data: () => {
    return {
      present: [],
      search: '',
      EXCLUDE_HEADERS: ['Id', 'attributes'],
    };
  },

  computed: {
    ...mapState({
      members: (state) => state.memberList,
    }),
    _headers() {
      return Object.keys(this.members[0]).map((k) => {
        return {
          text: k,
          value: k,
        };
      });
    },
    headers() {
      if (this._headers == undefined) return [];
      return this._headers.filter(
        (k) => !this.EXCLUDE_HEADERS.includes(k.text),
      );
    },
  },

  methods: {
    ...mapMutations(['replacePresentList']),
  },

  watch: {
    present(val) {
      this.replacePresentList(val.map((member) => member.Id));
    },
  },
};
</script>

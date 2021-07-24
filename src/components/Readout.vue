<template>
  <v-container>
    <v-row>
      <v-col>
        <v-card>
          <v-card-title>Attendance taken</v-card-title>
          <v-card-text>{{ attendanceWasTaken }}</v-card-text>
        </v-card>
      </v-col>
      <v-col>
        <v-card>
          <v-card-title>Proxies assigned</v-card-title>
          <v-card-text>{{ proxiesWereAssigned }}</v-card-text>
        </v-card>
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <h2>Members</h2>
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <v-card>
          <v-card-title>Total</v-card-title>
          <v-card-text>{{ total }}</v-card-text>
        </v-card>
      </v-col>
      <v-col>
        <v-card>
          <v-card-title>Present</v-card-title>
          <v-card-text>{{ present }}</v-card-text>
        </v-card>
      </v-col>
      <v-col>
        <v-card>
          <v-card-title>Represented</v-card-title>
          <v-card-text>{{ represented }}</v-card-text>
        </v-card>
      </v-col>
    </v-row>
    <v-row>
      <v-col><h2>Quorum requirements</h2></v-col>
    </v-row>
    <v-row>
      <v-col>
        <v-card>
          <v-card-title>Members</v-card-title>
          <v-card-subtitle>
            <v-chip v-if="haveMembersQuorum" color="success"> Quorum </v-chip>
            <v-chip v-else color="error">No Quorum</v-chip>
          </v-card-subtitle>
          <v-card-text>
            <v-list>
              <v-list-item two-line>
                <v-list-item-content>
                  <v-list-item-title> {{ present }} present </v-list-item-title>
                  <v-list-item-subtitle>
                    of {{ membersQuorumPresentThreshold }} required
                  </v-list-item-subtitle>
                </v-list-item-content>
              </v-list-item>
              <v-list-item two-line>
                <v-list-item-content>
                  <v-list-item-title>
                    {{ present + represented }} present or represented
                  </v-list-item-title>
                  <v-list-item-subtitle>
                    of {{ membersQuorumPresentOrRepresentedThreshold }}
                    required
                  </v-list-item-subtitle>
                </v-list-item-content>
              </v-list-item>
            </v-list>
          </v-card-text>
        </v-card>
      </v-col>
      <v-col>
        <v-card>
          <v-card-title>Directors</v-card-title>
          <v-card-subtitle>
            <v-chip v-if="haveDirectorsQuorum" color="success"> Quorum </v-chip>
            <v-chip v-else color="error">No Quorum</v-chip>
          </v-card-subtitle>
          <v-card-text>
            <v-list>
              <v-list-item two-line>
                <v-list-item-content>
                  <v-list-item-title> {{ present }} present </v-list-item-title>
                  <v-list-item-subtitle>
                    of {{ directorsQuorumThreshold }} required
                  </v-list-item-subtitle>
                </v-list-item-content>
              </v-list-item>
            </v-list>
          </v-card-text>
        </v-card>
      </v-col>
      <v-col>
        <v-card>
          <v-card-title>Passing thresholds</v-card-title>
          <v-card-text>
            <v-list>
              <v-list-item three-line>
                <v-list-item-content>
                  <v-list-item-title>
                    <v-chip v-if="canElectMembers" small color="success">
                      Can
                    </v-chip>
                    <v-chip v-else small color="warning">Cannot</v-chip>
                    elect members
                  </v-list-item-title>
                  <v-list-item-subtitle>
                    {{ present + represented }} present or represented
                  </v-list-item-subtitle>
                  <v-list-item-subtitle>
                    of {{ membershipElectionThreshold }} required
                  </v-list-item-subtitle>
                </v-list-item-content>
              </v-list-item>
              <v-list-item three-line>
                <v-list-item-content>
                  <v-list-item-title>
                    <v-chip v-if="canPassBylawsAmendment" small color="success">
                      Can
                    </v-chip>
                    <v-chip v-else small color="warning">Cannot</v-chip>
                    amend the bylaws</v-list-item-title
                  >
                  <v-list-item-subtitle>
                    {{ present + represented }} present or represented
                  </v-list-item-subtitle>
                  <v-list-item-subtitle>
                    of {{ bylawsAmendmentThreshold }} required
                  </v-list-item-subtitle>
                </v-list-item-content>
              </v-list-item>
              <v-list-item three-line>
                <v-list-item-content>
                  <v-list-item-title>
                    <v-chip
                      v-if="canPassConstitutionalAmendment"
                      small
                      color="success"
                    >
                      Can
                    </v-chip>
                    <v-chip v-else small color="warning">Cannot</v-chip>
                    amend the constitution</v-list-item-title
                  >
                  <v-list-item-subtitle>
                    {{ present + represented }} present or represented
                  </v-list-item-subtitle>
                  <v-list-item-subtitle>
                    of {{ constitutionalAmendmentThreshold }} required
                  </v-list-item-subtitle>
                </v-list-item-content>
              </v-list-item>
            </v-list>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <v-card>
          <v-card-title>Transcript</v-card-title>
          <v-card-text>
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
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
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

      'membersQuorumPresentThreshold',
      'membersQuorumPresentOrRepresentedThreshold',
      'haveMembersQuorum',

      'membershipElectionThreshold',
      'canElectMembers',

      'bylawsAmendmentThreshold',
      'canPassBylawsAmendment',

      'constitutionalAmendmentThreshold',
      'canPassConstitutionalAmendment',

      'directorsQuorumThreshold',
      'haveDirectorsQuorum',

      'attendanceWasTaken',
      'proxiesWereAssigned',
    ]),
  },
};
</script>

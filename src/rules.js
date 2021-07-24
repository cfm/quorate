// eslint-disable-next-line no-unused-vars
function membersQuorumPresentThreshold(present, represented, total) {
  const PRESENT_THRESHOLD = 1 / 3;
  return Math.ceil(PRESENT_THRESHOLD * total);
}

// eslint-disable-next-line no-unused-vars
function membersQuorumPresentOrRepresentedThreshold(
  present,
  represented,
  total,
) {
  const PRESENT_OR_REPRESENTED_THRESHOLD = 2 / 3;
  return Math.ceil(PRESENT_OR_REPRESENTED_THRESHOLD * total);
}

// eslint-disable-next-line no-unused-vars
function membershipElectionThreshold(present, represented, total) {
  const THRESHOLD = 0.75;
  return Math.ceil(THRESHOLD * total);
}

// eslint-disable-next-line no-unused-vars
function bylawsAmendmentThreshold(present, represented, total) {
  const THRESHOLD = 0.75;
  return Math.ceil(THRESHOLD * total);
}

// eslint-disable-next-line no-unused-vars
function constitutionalAmendmentThreshold(present, represented, total) {
  const THRESHOLD = 0.85;
  return Math.ceil(THRESHOLD * total);
}

// eslint-disable-next-line no-unused-vars
function directorsQuorumThreshold(present, represented, total) {
  return Math.ceil(5 + (total - 15) / 10);
}

export {
  membersQuorumPresentThreshold,
  membersQuorumPresentOrRepresentedThreshold,
  membershipElectionThreshold,
  bylawsAmendmentThreshold,
  constitutionalAmendmentThreshold,
  directorsQuorumThreshold,
};

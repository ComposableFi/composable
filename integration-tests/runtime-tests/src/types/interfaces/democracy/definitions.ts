export default {
  rpc: {},
  types: {
    PalletDemocracyVoteVoting: {
      direct: {
        votes: "Vec<Null>",
        delegations: {
          votes: "u128",
          capital: "u128",
          prior: "Null"
        }
      }
    },
    PalletDemocracyVoteThreshold: "Null",
    PalletDemocracyPreimageStatus: "Null",
    PalletDemocracyReferendumInfo: "Null",
    PalletPreimageRequestStatus: "Null",
    PalletDemocracyReleases: "Null",
  }
};

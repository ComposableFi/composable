export default {
  rpc: {},
  types: {
    PalletDemocracyVoteVoting: {
      _enum: {
        delegating: {
          balance: "Balance",
          target: "AccountId32",
          conviction: "Null",
          delegations: {
            votes: "Null",
            capital: "Null"
          },
          prior: "Null"
        },
        direct: {
          votes: "Vec<Null>",
          delegations: {
            votes: "u128",
            capital: "u128",
            prior: "Null"
          }
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

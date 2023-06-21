export default {
  rpc: {},
  types: {
    FrameSystemAccountInfo: {
      nonce: "Null",
      consumers: "Null",
      providers: "Null",
      sufficients: "Null",
      data: {
        free: "u128",
        reserved: "u128",
        miscFrozen: "u128",
        feeFrozen: "u128"
      }
    },
    FrameSystemLastRuntimeUpgradeInfo: "Null",
    FrameSystemPhase: "Null",
    FrameSystemEventRecord: {
      phase: "Null",
      event: {
        section: "Null",
        method: "Null"
      },
      topics: "Null"
    },
    FrameSystemLimitsBlockWeights: "Null",
    FrameSystemLimitsBlockLength: "Null"
  }
};

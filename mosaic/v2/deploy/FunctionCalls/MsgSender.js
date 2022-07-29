const { deployUpgradable } = require("../utils");

module.exports = async () => {
  await deployUpgradable("MsgSender");
};

module.exports.tags = ["MsgSender_Deploy"];

const { deployNonUpgradable } = require("../utils");

module.exports = async () => {
  await deployNonUpgradable("FundKeeper");
};

module.exports.tags = ["FundKeeper_deploy"];

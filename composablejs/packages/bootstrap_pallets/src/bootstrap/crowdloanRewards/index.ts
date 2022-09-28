import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import config from "@composable/bootstrap_pallets/constants/config.json";
import rewards from "@composable/bootstrap_pallets/constants/rewards.json";
import { PalletCrowdloanRewardsModelsRemoteAccount } from "@composable/types";
import { u128, u32 } from "@polkadot/types";
import { addFundsToCrowdloan, initialize, logger, sendAndWaitForSuccess, toChainUnits } from "../..";
import BigNumber from "bignumber.js";

function toPalletCrowdloanRewardsModelsRemoteAccount(
  api: ApiPromise,
  account: string,
  reward: string,
  vestingPeriod: string
): [PalletCrowdloanRewardsModelsRemoteAccount, u128, u32] {
  if (account.startsWith("0x")) {
    return [
      api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
        Ethereum: account
      }),
      api.createType("u128", toChainUnits(reward).toString()),
      api.createType("u32", vestingPeriod)
    ] as [PalletCrowdloanRewardsModelsRemoteAccount, u128, u32];
  } else {
    return [
      api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
        RelayChain: api.createType("AccountId32", account).toU8a()
      }),
      api.createType("u128", toChainUnits(reward).toString()),
      api.createType("u32", vestingPeriod)
    ] as [PalletCrowdloanRewardsModelsRemoteAccount, u128, u32];
  }
}

export async function bootstrapCrowdloanRewards(api: ApiPromise, walletSudo: KeyringPair): Promise<void> {
  const allRewards = Object.entries(rewards);

  const STEP = 5;
  for (let i = 0; i < allRewards.length; i += STEP) {
    let accountsOfBatch: [PalletCrowdloanRewardsModelsRemoteAccount, u128, u32][] = [];
    
    let accIndex = i;
    let amount = new BigNumber(0);
    while (accIndex < allRewards.length && accIndex < STEP + i) {
      amount = amount.plus(allRewards[accIndex][1]);
      accountsOfBatch.push(toPalletCrowdloanRewardsModelsRemoteAccount(
        api,
        allRewards[accIndex][0],
        allRewards[accIndex][1],
        config.crowdloanRewards.vestingPeriod
      ))
      accIndex = accIndex + 1;
    }

    logger.info(`Adding Funds to Crowdloan: ${amount.toString()}`);
    await addFundsToCrowdloan(
      api,
      walletSudo,
      api.createType("u128", toChainUnits(amount).toString()),
      config.crowdloanRewards.palletAccountId
    );

    logger.info(`Populating Accounts: ${accIndex}`);
    await sendAndWaitForSuccess(
      api,
      walletSudo,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.crowdloanRewards.populate(accountsOfBatch))
    );
  }

  logger.info(`Initializing Crowdloan Rewards`);
  await initialize(api, walletSudo);
}

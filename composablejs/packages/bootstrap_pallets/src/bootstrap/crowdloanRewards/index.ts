import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import config from "@composable/bootstrap_pallets/constants/config.json";
import { PalletCrowdloanRewardsModelsRemoteAccount } from "@composable/types";
import { u128, u32 } from "@polkadot/types";
import { addFundsToCrowdloan, initialize, populate } from "../..";
import { BN } from "bn.js";
import { sendAndWaitForWithBatch } from "@composable/bootstrap_pallets/lib/polkadot/sendAndWaitForWithBatch";

export async function bootstrapCrowdloanRewards(api: ApiPromise, walletSudo: KeyringPair): Promise<void> {
  
  const rewardsPerAccount = api.createType("u128", config.crowdloanRewards.rewardsPerAccount);
  const vestingPeriod = api.createType("u32", config.crowdloanRewards.vestingPeriod);

  const allAccounts = config.crowdloanRewards.ethereumContributors.concat(
    config.crowdloanRewards.relayChainContributors
  ).map(account => {
      if (account.startsWith("0x")) {
        return [
          api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
            Ethereum: account
          }),
          rewardsPerAccount,
          vestingPeriod
        ] as [PalletCrowdloanRewardsModelsRemoteAccount, u128, u32];
      } else {
        [
          api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
            RelayChain: api.createType("AccountId32", account).toU8a()
          }),
          rewardsPerAccount,
          vestingPeriod
        ] as [PalletCrowdloanRewardsModelsRemoteAccount, u128, u32];
      }
  }) as [PalletCrowdloanRewardsModelsRemoteAccount, u128, u32][];

  // await addFundsToCrowdloan(api, walletSudo, api.createType("u128", rewardsPerAccount.mul(new BN(allAccounts.length))), config.crowdloanRewards.palletAccountId);
  
  await populate(api, walletSudo, allAccounts);

  await initialize(api, walletSudo);
}

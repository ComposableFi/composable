import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import config from "@composable/bootstrap_pallets/constants/config.json";
import { PalletCrowdloanRewardsModelsRemoteAccount } from "@composable/types";
import { u128, u32 } from "@polkadot/types";
import { addFundsToCrowdloan, initialize } from "../..";
import { BN } from "bn.js";
import { sendAndWaitForWithBatch } from "@composable/bootstrap_pallets/lib/polkadot/sendAndWaitForWithBatch";

export async function bootstrapCrowdloanRewards(api: ApiPromise, walletSudo: KeyringPair): Promise<void> {
  const step = 10;
  let index = 0;

  const txCalls = [];
  const allAcounts = config.crowdloanRewards.ethereumContributors.concat(
    config.crowdloanRewards.relayChainContributors
  );

  const rewardsPerAccount = api.createType("u128", config.crowdloanRewards.rewardsPerAccount);
  const vestingPeriod = api.createType("u32", config.crowdloanRewards.vestingPeriod);

  while (index < allAcounts.length) {
    const accounts = allAcounts.slice(index, index + step > allAcounts.length ? allAcounts.length : step);
    const batchAccounts = accounts.map(account => {
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

    txCalls.push(api.tx.sudo.sudo(api.tx.crowdloanRewards.populate(batchAccounts)));
    index += step;
  }

  await addFundsToCrowdloan(
    api,
    walletSudo,
    api.createType("u128", rewardsPerAccount.mul(new BN(allAcounts.length))),
    config.crowdloanRewards.palletAccountId
  );

  await sendAndWaitForWithBatch(api, walletSudo, api.events.sudo.Sudid.is, txCalls, false);

  await initialize(api, walletSudo);
}

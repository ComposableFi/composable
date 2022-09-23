import { u128, u32 } from "@polkadot/types-codec";
import { KeyringPair } from "@polkadot/keyring/types";
import { ethers } from "ethers";
import { ApiPromise } from "@polkadot/api";
import { PalletCrowdloanRewardsModelsRemoteAccount } from "@composable/types";
import { mintAssetsToWallets } from "../assets/extrinsics";
import BigNumber from "bignumber.js";
import { sendAndWaitForSuccess, sendUnsignedAndWaitForSuccess } from "@composable/bootstrap_pallets/lib";
import { toHexString } from "@composable/bootstrap_pallets/utils";

export const associateKSM = async (api: ApiPromise, contributorAccount: KeyringPair, rewardAccount: KeyringPair) => {
  const message = `<Bytes>picasso-${toHexString(rewardAccount.publicKey)}</Bytes>`;
  const signature = await contributorAccount.sign(message);
  const proof = {
    RelayChain: [contributorAccount.publicKey, { Sr25519: signature }]
  };

  return sendUnsignedAndWaitForSuccess(
    api,
    api.events.crowdloanRewards.Associated.is,
    api.tx.crowdloanRewards.associate(rewardAccount.publicKey, proof)
  );
};

export const associateEth = async (api: ApiPromise, signer: ethers.Signer, rewardAccount: KeyringPair) => {
  const message = `picasso-${toHexString(rewardAccount.publicKey)}`;
  const signature = await signer.signMessage(message);
  const proof: any = {
    proof: { Ethereum: signature }
  };

  return sendUnsignedAndWaitForSuccess(
    api,
    api.events.crowdloanRewards.Associated.is,
    api.tx.crowdloanRewards.associate(rewardAccount.publicKey, proof)
  );
};

export const initialize = async (api: ApiPromise, sudoAccount: KeyringPair) => {
  return await sendAndWaitForSuccess(
    api,
    sudoAccount,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(api.tx.crowdloanRewards.initialize())
  );
};

export const populate = async (
  api: ApiPromise,
  walletSudo: KeyringPair,
  relayAccounts: string[],
  ethAccounts: string[],
  rewardsPerAccount: string,
  vestingPeriod: string
) => {
  const _vestingPeriod = api.createType("u32", vestingPeriod);
  const _rewardsPerAccount = api.createType("u128", rewardsPerAccount);

  const relayContributors = relayAccounts.map(
    account =>
      [
        api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
          RelayChain: api.createType("AccountId32", account).toU8a()
        }),
        _rewardsPerAccount,
        _vestingPeriod
      ] as [PalletCrowdloanRewardsModelsRemoteAccount, u128, u32]
  );

  const ethereumContributors = ethAccounts.map(
    account =>
      [
        api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
          Ethereum: account
        }),
        _rewardsPerAccount,
        _vestingPeriod
      ] as [PalletCrowdloanRewardsModelsRemoteAccount, u128, u32]
  );

  const sliced = relayContributors.concat(ethereumContributors);

  return await sendAndWaitForSuccess(
    api,
    walletSudo,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(api.tx.crowdloanRewards.populate(sliced))
  );
};

export const addFundsToCrowdloan = async (
  api: ApiPromise,
  walletSudo: KeyringPair,
  amount: u128,
  palletAccountId: string
) => {
  await mintAssetsToWallets(api, [walletSudo], walletSudo, ["1"], new BigNumber(amount.toString()));
  await sendAndWaitForSuccess(
    api,
    walletSudo,
    api.events.balances.Transfer.is,
    api.tx.assets.transfer(1, palletAccountId, amount, true)
  );
};

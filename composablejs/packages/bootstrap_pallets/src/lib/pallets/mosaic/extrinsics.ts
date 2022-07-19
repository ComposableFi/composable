import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { sendAndWaitForSuccess } from "@bootstrap-pallets/lib";

export const setRelayer = async (api: ApiPromise, relayerAccount: KeyringPair, sudoAccount: KeyringPair) => {
  return sendAndWaitForSuccess(
    api,
    sudoAccount,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(api.tx.mosaic.setRelayer(relayerAccount.publicKey))
  );
};

// todo: add dynamic parameters
export const setNetwork = async (api: ApiPromise, relayerAccount: KeyringPair) => {
  return sendAndWaitForSuccess(
    api,
    relayerAccount,
    api.events.mosaic.NetworksUpdated.is,
    api.tx.mosaic.setNetwork(1, {
      enabled: true,
      maxTransferSize: api.createType("u128", 100_000_000_000_000)
    })
  );
};

// to-do add dynamic parameters
// all params are from repo benchamarks
export const setBudget = async (assetId: number, api: ApiPromise, sudoAccount: KeyringPair) => {
  return sendAndWaitForSuccess(
    api,
    sudoAccount,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(
      api.tx.mosaic.setBudget(api.createType("u128", assetId), api.createType("u128", 100_000_000_000_000), {
        Linear: api.createType("u128", 5)
      })
    )
  );
};

// export const timeLockedMint = async (
//   api: ApiPromise,
//   assetId: number,
//   accountTo: KeyringPair,
//   amount: number,
//   lockTime: number, // blocknumber
//   id: string,
//   relayerAccount: KeyringPair,
// ) => {
//   return sendAndWaitForSuccess(
//     api,
//     relayerAccount,
//     api.events.mosaic.TransferInto.is,
//     api.tx.mosaic.timelockedMint(assetId, accountTo.publicKey, amount, lockTime, id)
//   );
// }

export const claimTo = async (api: ApiPromise, assetId: number, claimerAccount: KeyringPair) => {
  return sendAndWaitForSuccess(
    api,
    claimerAccount,
    api.events.mosaic.TransferClaimed.is,
    api.tx.mosaic.claimTo(assetId, claimerAccount.publicKey)
  );
};

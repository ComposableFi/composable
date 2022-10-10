import 'react-app-polyfill/ie11';
import { useEffect, useState } from 'react';
import { useSigner, activate, SupportedWalletId, useExecutor, useExtrinsicStore, useSubstrateNetwork, SubstrateChainApi, useHasInitialized } from '../../src';
import BigNumber from 'bignumber.js';

const APP_NAME = "Demo App";

export const Transfers = () => {
  const signer = useSigner();
  const network: SubstrateChainApi = useSubstrateNetwork("picasso");
  const hasInitialized = useHasInitialized();
  const executor = useExecutor();
  const extrinsics = useExtrinsicStore((state) => state.extrinsics);
  const [_to, setTo] = useState<string | undefined>(undefined);
  const [_from, setFrom] = useState<string | undefined>(undefined);

  useEffect(() => {
    if (hasInitialized) {
      activate(APP_NAME, SupportedWalletId.Polkadotjs).catch(console.error);
    }
  }, [hasInitialized]);

  const { connectedAccounts, api } = network;
  useEffect(() => {
    if (connectedAccounts.length > 0) {
      setFrom(connectedAccounts[0].address)
    }
  }, [connectedAccounts]);

  const onTransfer = async () => {
    if (_from && _to && executor && signer) {
      const decimals = new BigNumber(10).pow(12); // Substrate default decimals
      const transferAmount = new BigNumber(0.0001).times(decimals);

      executor.execute(
        //@ts-ignore
        api.tx.balances.transfer(_to, transferAmount.toString()),
        _from,
        api,
        signer,
        (txHash) => {
          console.log("Ready: ", txHash);
        },
        (txHash) => {
          console.log("Finalized: ", txHash);
        }
      );
    }
  };

  useEffect(() => {
    console.log("Extrinsics Update", extrinsics);
  }, [extrinsics]);

  return (
    <div>
      <input
        onChange={(evt) => {
          setTo(evt.target.value);
        }}
        type="text"
      />
      <button onClick={onTransfer}>Transfer</button>
    </div>
  );
};

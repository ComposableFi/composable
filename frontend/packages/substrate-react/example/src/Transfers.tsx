import 'react-app-polyfill/ie11';
import { useEffect, useState } from 'react';
import {
  SupportedWalletId,
  useDotSamaContext,
  useExecutor,
  useParachainApi,
} from '../../src/index';
import BigNumber from 'bignumber.js';
import { useExtrinsicStore } from "../../src/extrinsics/store/extrinsics/extrinsics.slice";

const APP_NAME = "Demo App";

export const Transfers = () => {
  const { activate, signer } = useDotSamaContext();
  const { parachainApi, accounts } = useParachainApi("picasso");
  const executor = useExecutor();
  const extrinsics = useExtrinsicStore((state) => state.extrinsics);
  const [_to, setTo] = useState<string | undefined>(undefined);
  const [_from, setFrom] = useState<string | undefined>(undefined);

  useEffect(() => {
    if (activate) {
      activate(SupportedWalletId.Polkadotjs);
    }
  }, [activate]);

  useEffect(() => {
    if (accounts.length) {
      setFrom(accounts[0].address);
    }
  }, [accounts]);

  const onTransfer = async () => {
    if (parachainApi && _from && _to && executor && signer) {
      const decimals = new BigNumber(10).pow(12); // Substrate default decimals
      const transferAmount = new BigNumber(0.0001).times(decimals);

      executor.execute(
        //@ts-ignore
        parachainApi.tx.balances.transfer(_to, transferAmount.toString()),
        _from,
        parachainApi,
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

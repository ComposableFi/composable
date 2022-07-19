import 'react-app-polyfill/ie11';
import { useEffect, useState } from 'react';
import { web3Enable, web3FromAddress } from '@polkadot/extension-dapp';
import {
  useDotSamaContext,
  useExecutor,
  useParachainApi,
} from '../../src/index';
import BigNumber from 'bignumber.js';
import useStore from '../../src/extrinsics/store/useStore';

const APP_NAME = 'Demo App';

export const Transfers = () => {
  const { activate } = useDotSamaContext();
  const { parachainApi, accounts } = useParachainApi('picasso');
  const executor = useExecutor();
  const { extrinsics } = useStore();
  const [_to, setTo] = useState<string | undefined>(undefined);
  const [_from, setFrom] = useState<string | undefined>(undefined);

  useEffect(() => {
    if (parachainApi && activate) {
      activate();
    }
  }, [parachainApi, activate]);

  useEffect(() => {
    if (accounts.length) {
      setFrom(accounts[0].address);
    }
  }, [accounts]);

  const onTransfer = async () => {
    if (parachainApi && _from && _to && executor) {
      await web3Enable(APP_NAME);
      const injector = await web3FromAddress(_from);
      const decimals = new BigNumber(10).pow(12); // Substrate default decimals
      const transferAmnt = new BigNumber(0.0001).times(decimals);

      executor.execute(
        //@ts-ignore
        _api.tx.transfer(_to, transferAmnt.toString()),
        _from,
        parachainApi,
        injector.signer,
        txHash => {
          console.log('Ready: ', txHash);
        },
        txHash => {
          console.log('Finalised: ', txHash);
        }
      );
    }
  };

  useEffect(() => {
    console.log('Extrinsics Update', extrinsics);
  }, [extrinsics]);

  return (
    <div>
      <input
        onChange={evt => {
          setTo(evt.target.value);
        }}
        type="text"
      />
      <button onClick={onTransfer}>Transfer</button>
    </div>
  );
};

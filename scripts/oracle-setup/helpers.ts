import { ApiPromise, SubmittableResult, WsProvider } from "@polkadot/api";
import { AddressOrPair, SubmittableExtrinsic } from "@polkadot/api/types";
import { Keyring } from "@polkadot/keyring";

export const getApi = async (nodeName: string, endpoint?: string): Promise<ApiPromise> => {
	const wsProvider = endpoint
	  ? new WsProvider(endpoint + nodeName)
	  : new WsProvider("ws://127.0.0.1:9944");

	const api = await ApiPromise.create({ provider: wsProvider });
    await api.isReady;
	return api;
  };


export const getKeypair = (mneumonic: string): AddressOrPair => {
	try {
	  const keyring = new Keyring({ type: "sr25519" });
	  return keyring.addFromUri("//" + mneumonic);
	} catch (e: any) {
	  console.log("error setting up keypair");
	  throw new Error(e.message);
	}
  };

/**
 * Signs and sends the given `call` from `sender` and waits for the transaction to be included in a block.
 * @param api api object
 * @param call a call that can be submitted to the chain
 * @param sender the sender of the transaction
 */
 export const sendAndWait = (
	api: ApiPromise,
	call: SubmittableExtrinsic<"promise">,
	sender: AddressOrPair
  ): Promise<undefined> => {
	return new Promise<undefined>((resolve, reject) => {
	  call
		.signAndSend(sender, (res: SubmittableResult) => {
		  const { dispatchError, status } = res;
  
		  if (dispatchError) {
			if (dispatchError.isModule) {
			  // for module errors, we have the section indexed, lookup
			  const decoded = api.registry.findMetaError(dispatchError.asModule);
			  const { name, section } = decoded;

			  reject(Error(dispatchError.toString()));
			} else {
			  reject(Error(dispatchError.toString()));
			}
		  }
  
		  if (status.isInBlock || status.isFinalized) {
			resolve(undefined);
		  }
		})
		.catch((e) => {
		  reject(Error(e.message));
		});
	});
  }


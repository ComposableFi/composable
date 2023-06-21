import { ApiPromise } from "@polkadot/api";
import { SubmittableExtrinsic } from "@polkadot/api/types";
import { KeyringPair } from "@polkadot/keyring/types";
import { ISubmittableResult } from "@polkadot/types/types";

export const sendWait = (
  api: ApiPromise,
  call: SubmittableExtrinsic<"promise">,
  sender: KeyringPair
): Promise<ISubmittableResult> => {
  return new Promise((res, rej) => {
    call.signAndSend(sender, { nonce: -1 }, result => {
      if (result.dispatchError) {
        let errorMessage = "Error";
        if (result.dispatchError.isModule) {
          const decoded = api.registry.findMetaError(result.dispatchError.asModule);
          const { docs, name, section } = decoded;

          errorMessage = `${section}.${name}: ${docs.join(" ")}`;
        } else {
          errorMessage = result.dispatchError.toString();
        }

        rej(errorMessage);
      }

      if (result.isFinalized) {
        res(result);
      }
    });
  });
};

import { ApiPromise, SubmittableResult } from "@polkadot/api";
import {
  AddressOrPair,
  Signer,
  SubmittableExtrinsic,
} from "@polkadot/api/types";
import { EventRecord } from "@polkadot/types/interfaces/system";
import {
  ExtrinsicMetadata,
  ExtrinsicSlice,
} from "./store/extrinsics/extrinsics.types";

interface TransactionExecutor {
  execute(
    call: SubmittableExtrinsic<"promise">,
    sender: AddressOrPair,
    api: ApiPromise,
    signer: Signer,
    onTxReady: (txHash: string) => void | undefined,
    onTxFinalized: (txHash: string, events: EventRecord[]) => void | undefined,
    onTxError?: (errorMessage: string) => void
  ): Promise<void>;

  executeUnsigned(
    call: SubmittableExtrinsic<"promise">,
    api: ApiPromise,
    onTxReady: (txHash: string) => void | undefined,
    onTxFinalized: (txHash: string) => void | undefined
  ): Promise<void>;
}

export class Executor implements TransactionExecutor {
  private addExtrinsic: ExtrinsicSlice["addExtrinsic"];
  private addBlockHash: ExtrinsicSlice["addBlockHash"];
  private updateExtrinsicStatus: ExtrinsicSlice["updateExtrinsicStatus"];
  private updateExtrinsicError: ExtrinsicSlice["updateExtrinsicError"];

  constructor(
    addExtrinsic: ExtrinsicSlice["addExtrinsic"],
    addBlockHash: ExtrinsicSlice["addBlockHash"],
    updateExtrinsicStatus: ExtrinsicSlice["updateExtrinsicStatus"],
    updateExtrinsicError: ExtrinsicSlice["updateExtrinsicError"]
  ) {
    this.addExtrinsic = addExtrinsic;
    this.addBlockHash = addBlockHash;
    this.updateExtrinsicStatus = updateExtrinsicStatus;
    this.updateExtrinsicError = updateExtrinsicError;
  }

  /**
   * Execute an API Call (legacy or not?)
   * @param call a submittable extrinsic from Polkadot/api
   * @param sender address of the user
   * @param api polkadot api itself
   * @param signer signer from an extension wallet
   * @param onTxFinalized this should be optional
   */
  async execute(
    call: SubmittableExtrinsic<"promise">,
    sender: AddressOrPair,
    api: ApiPromise,
    signer: Signer,
    onTxReady: (txHash: string) => void | undefined,
    onTxFinalized: (txHash: string, events: EventRecord[]) => void | undefined,
    onTxError?: (errorMessage: string) => void | undefined
  ): Promise<void> {
    const unsub = await call.signAndSend(sender, { signer }, (txResult) => {
      const txHash = txResult.txHash.toString().toLowerCase();

      if (txResult.status.isReady) {
        this.onReady(call, txResult, sender, true);

        if (onTxReady) {
          onTxReady(txHash);
        }
      }

      if (txResult.dispatchError) {
        const error = this.onDispatchError(txResult, api);
        if (onTxError) onTxError(error);
        unsub();
      }

      if (txResult.isInBlock) {
        this.onFinalized(txHash);

        if (onTxFinalized) {
          onTxFinalized(txHash, txResult.events);
        }
        unsub();
      }
    });
  }

  /**
   * This should be used to fetch transaction fees based on call
   * @param call
   * @param sender
   * @param signer
   */
  async paymentInfo(
    call: SubmittableExtrinsic<"promise">,
    sender: AddressOrPair,
    signer: Signer
  ) {
    return call.paymentInfo(sender, { signer });
  }

  async executeUnsigned(
    call: SubmittableExtrinsic<"promise">,
    api: ApiPromise,
    onTxReady: (txHash: string) => void | undefined,
    onTxFinalized: (txHash: string) => void | undefined
  ): Promise<void> {
    const unsub = await call.send((txResult) => {
      const txHash = txResult.txHash.toString().toLowerCase();
      if (txResult.status.isReady) {
        this.onReady(call, txResult, "", false);
        if (onTxReady) onTxReady(txHash);
      }

      if (txResult.status.isInBlock) {
        this.onBlockInclusion(txResult);
      }

      if (txResult.dispatchError) {
        this.onDispatchError(txResult, api);
        unsub();
      }

      if (txResult.isFinalized && !txResult.dispatchError) {
        this.onFinalized(txHash);
        if (onTxFinalized) onTxFinalized(txHash);
        unsub();
      }
    });
  }

  private async onReady(
    call: SubmittableExtrinsic<"promise">,
    txResult: SubmittableResult,
    sender: AddressOrPair,
    isSigned: boolean
  ) {
    const serialized: any = call.toHuman();
    const txHash = txResult.txHash.toString().toLowerCase();
    const payload: ExtrinsicMetadata = {
      hash: txHash,
      method: serialized.method.method,
      section: serialized.method.section,
      sender: sender.toString(),
      args: serialized.method.args,
      dispatchError: undefined,
      status: "isReady",
      isSigned,
      timestamp: Date.now(),
    };

    this.addExtrinsic(txHash, payload);
  }

  private onDispatchError(txResult: SubmittableResult, api: ApiPromise) {
    let errorMessage = ``;
    let txHash = txResult.txHash.toString().toLowerCase();

    if (txResult.dispatchError) {
      if (txResult.dispatchError.isModule) {
        const decoded = api.registry.findMetaError(
          txResult.dispatchError.asModule
        );
        const { docs, name, section } = decoded;

        errorMessage = `${section}.${name}: ${docs.join(" ")}`;
      } else {
        errorMessage = txResult.dispatchError.toString();
      }
    }

    this.updateExtrinsicError(txHash, errorMessage);
    return errorMessage;
  }

  private async onBlockInclusion(txResult: SubmittableResult) {
    let txHash = txResult.txHash.toString().toLowerCase();
    let blockHash = txResult.status.asInBlock.toString().toLowerCase();

    this.addBlockHash(txHash, blockHash);
  }

  private async onFinalized(txHash: string) {
    this.updateExtrinsicStatus(txHash, "isFinalized");
  }
}

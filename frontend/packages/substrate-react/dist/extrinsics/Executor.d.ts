import { ApiPromise } from '@polkadot/api';
import { SubmittableExtrinsic, AddressOrPair, Signer } from '@polkadot/api/types';
import { EventRecord } from '@polkadot/types/interfaces/system';
import { ExtrinsicSlice } from './store/extrinsics/extrinsics.types';
interface TransactionExecutor {
    execute(call: SubmittableExtrinsic<'promise'>, sender: AddressOrPair, api: ApiPromise, signer: Signer, onTxReady: (txHash: string) => void | undefined, onTxFinalized: (txHash: string, events: EventRecord[]) => void | undefined, onTxError?: (errorMessage: string) => void): Promise<void>;
    executeUnsigned(call: SubmittableExtrinsic<'promise'>, api: ApiPromise, onTxReady: (txHash: string) => void | undefined, onTxFinalized: (txHash: string) => void | undefined): Promise<void>;
}
declare class Executor implements TransactionExecutor {
    private addExtrinsic;
    private addBlockHash;
    private updateExstrinsicStatus;
    private updateExtrinsicError;
    constructor(addExtrinsic: ExtrinsicSlice['addExtrinsic'], addBlockHash: ExtrinsicSlice['addBlockHash'], updateExstrinsicStatus: ExtrinsicSlice['updateExtrinsicStatus'], updateExtrinsicError: ExtrinsicSlice['updateExtrinsicError']);
    /**
     * Execute an API Call (legacy or not?)
     * @param call a submittable extrinsic from Polkadot/api
     * @param sender address of the user
     * @param api polkadot api itself
     * @param signer signer from an extension wallet
     * @param onTxFinalized this should be optional
     */
    execute(call: SubmittableExtrinsic<'promise'>, sender: AddressOrPair, api: ApiPromise, signer: Signer, onTxReady: (txHash: string) => void | undefined, onTxFinalized: (txHash: string, events: EventRecord[]) => void | undefined, onTxError?: (errorMessage: string) => void | undefined): Promise<void>;
    executeUnsigned(call: SubmittableExtrinsic<'promise'>, api: ApiPromise, onTxReady: (txHash: string) => void | undefined, onTxFinalized: (txHash: string) => void | undefined): Promise<void>;
    private onReady;
    private onDispatchError;
    private onBlockInclusion;
    private onFinalized;
}
export default Executor;

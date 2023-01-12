import type {Result, Option} from './support'

export interface Fee {
    fee: bigint
    lpFee: bigint
    ownerFee: bigint
    protocolFee: bigint
    assetId: bigint
}

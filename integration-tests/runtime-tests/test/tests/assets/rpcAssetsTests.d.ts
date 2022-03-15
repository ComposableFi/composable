import { CurrencyId } from '@composable/types/interfaces';
import { AnyNumber } from '@polkadot/types-codec/types';
export declare class RpcAssetsTests {
    static rpcAssetsTest(asset_id: CurrencyId | AnyNumber, publicKey: string | Uint8Array): Promise<import("@composable/types/interfaces").AssetsBalance>;
}

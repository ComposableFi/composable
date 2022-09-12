import { SafeRpcWrapper } from "@composable/types/interfaces";
import { ApiPromise } from "@polkadot/api";
export declare class RpcAssetsTests {
    static rpcAssetsTest(apiClient: ApiPromise, assetId: SafeRpcWrapper, publicKey: string | Uint8Array): Promise<import("@composable/types/interfaces").CustomRpcBalance>;
    static rpcListAssetsTest(apiClient: ApiPromise): Promise<import("@polkadot/types-codec").Vec<import("@composable/types/interfaces").Asset>>;
}

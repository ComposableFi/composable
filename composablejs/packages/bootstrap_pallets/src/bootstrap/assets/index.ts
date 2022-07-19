import { mintAssetsToAddress } from "@bootstrap-pallets/lib";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";

export async function bootstrapAssets(api: ApiPromise, walletSudo: KeyringPair, mint: [string, string, string][]): Promise<void> {
    for (const m of mint) {
        const [wallet, assetId, amount] = m;
        await mintAssetsToAddress(api, [wallet], walletSudo, [assetId], amount);
    }
}

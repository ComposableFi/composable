import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import type { KeyringPair } from "@polkadot/keyring/types";
import type { ISubmittableResult, RegistryTypes } from "@polkadot/types/types";

async function main() {
    const url = process.env.NODE_URL ?? "ws://localhost:9944";
    const paraId = Number(process.env.PARA_ID ?? 2087);
    const leaseCount = Number(process.env.LEASE_PERIOD ?? 365);
    console.log(`node url: ${url}, para id: ${paraId}, lease period: ${leaseCount} days`);

    const api = await createApi(url, {});

    await chainInfo(api);

    const keyring = new Keyring({ type: "sr25519" });
    const root = keyring.addFromUri("//Alice", { name: "Alice default" });
    await forceLease(api, root, paraId, 0, 0, leaseCount);
}

async function createApi(url: string, types: RegistryTypes | undefined): Promise<ApiPromise> {
    const provider = new WsProvider(url);
    return await ApiPromise.create({ provider, types });
}

async function chainInfo(api: ApiPromise) {
    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version(),
    ]);

    console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
}

async function forceLease(
    api: ApiPromise,
    sudoAcc: KeyringPair,
    paraId: number,
    amount: number,
    begin: number,
    count: number
) {
    return new Promise(async (resolvePromise, reject) => {
        await api.tx.sudo
            .sudo(api.tx.slots.forceLease(paraId, sudoAcc.address, amount, begin, count))
            .signAndSend(sudoAcc, ({ status }: ISubmittableResult) => {
                console.log(`Current status is ${status}`);
                if (status.isInBlock) {
                    console.log(`Transaction included at blockHash ${status.asInBlock}`);
                } else if (status.isFinalized) {
                    resolvePromise(0)
                }
            });
    })
}

main().catch(console.error).finally(() => process.exit());

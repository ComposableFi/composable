import { ApiPromise } from "@polkadot/api";

export class Apollo {
    api: ApiPromise;

    constructor(
        api: ApiPromise
    ) {
        this.api = api;
    }
}
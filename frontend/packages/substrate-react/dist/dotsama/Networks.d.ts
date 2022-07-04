import { ParachainId, ParachainNetwork, RelayChainId, RelaychainNetwork } from './types';
export declare const ParachainNetworks: {
    [parachainId in ParachainId]: ParachainNetwork;
};
export declare const RelayChainNetworks: {
    [relaychainId in RelayChainId]: RelaychainNetwork;
};
export declare const getParachainNetwork: (parachainId: ParachainId) => ParachainNetwork;
export declare const getRelaychainNetwork: (relaychainId: RelayChainId) => RelaychainNetwork;

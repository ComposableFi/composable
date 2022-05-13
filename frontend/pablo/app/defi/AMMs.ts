import { AMM, AmmId } from './types';
import { DEFI_CONFIG } from "./config";

export const AMM_IDS = DEFI_CONFIG.ammIds;
export const AMMs: { [key in AmmId]: AMM } = {
  uniswap: {
    id: 'uniswap',
    icon: '/tokens/eth-mainnet.svg',
    label: 'Uniswap',
  },
  sushiswap: {
    id: 'sushiswap',
    icon: '/tokens/weth-mainnet.svg',
    label: 'Sushiswap',
  },
  quickiswap: {
    id: 'quickiswap',
    icon: '/tokens/avalanche.svg',
    label: 'Quickswap',
  },
};
export const getAMM = (ammId: AmmId): AMM => AMMs[ammId];

export const getAMMOptions = (noneAMMLabel?: string) => ([
  ...(noneAMMLabel ? [{
    value: "none",
    label: noneAMMLabel,
    icon: undefined,
    disabled: true,
    hidden: true,
  }] : []),
  ...Object.keys(AMMs).map((amm_id) => ({
    value: amm_id,
    label: getAMM(amm_id as AmmId).label,
    icon: getAMM(amm_id as AmmId).icon,
  })),
]);
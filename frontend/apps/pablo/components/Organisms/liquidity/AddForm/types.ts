import { Asset, SubstrateNetworkId } from "shared";
import BigNumber from "bignumber.js";

export type InputConfig = {
  asset: Asset;
  balance: {
    free: BigNumber;
    locked: BigNumber;
  };
  chainId: SubstrateNetworkId;
};

export type Config = {
  asset: Asset;
  chainId: string;
  balance: {
    free: BigNumber;
    locked: BigNumber;
  };
};

export type Controlled = {
  onChange: (v: BigNumber) => void;
  value: BigNumber;
};

export type AssetDropdown = {
  assetDropdownItems: {
    value: string;
    label: string;
    icon?: string;
    disabled?: boolean;
    hidden?: boolean;
  }[];
};

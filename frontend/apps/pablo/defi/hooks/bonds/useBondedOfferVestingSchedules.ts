import { BondOffer, BondPrincipalAsset } from "@/defi/types";
import { useCallback, useEffect, useMemo, useState } from "react";
import { useAllLpTokenRewardingPools } from "@/store/hooks/useAllLpTokenRewardingPools";
import { MockedAsset } from "@/store/assets/assets.types";
import {
  AVERAGE_BLOCK_TIME,
  calculateBondROI,
  calculateVestingTime,
  decodeBondOffer,
  DEFAULT_NETWORK_ID,
  getBondPrincipalAsset,
  matchAssetByPicassoId,
} from "@/defi/utils";
import { useParachainApi } from "substrate-react";
import { useBlockInterval } from "../useBlockInterval";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";

export default function useBondedOfferVestingSchedules(bondOffer: BondOffer) {

}

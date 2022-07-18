import { useEffect } from "react";
import useStore from "@/store/useStore";

import { DEFAULT_NETWORK_ID, fetchBondOffers } from "@/defi/utils";
import { useParachainApi } from "substrate-react";

const Updater = () => {
  const { bondOffers, supportedAssets, apollo } = useStore();
//   const lpRewardingPools = useAllLpTokenRewardingPools();

  const { putBondOffers } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);

  useEffect(() => {
    if (parachainApi) {
      fetchBondOffers(parachainApi).then((decodedOffers) => {
        putBondOffers(decodedOffers);
      });
    }
  }, [parachainApi, putBondOffers]);

  useEffect(() => {
    const bondOfferIds = bondOffers.list.map(i => (i.offerId.toString()));

    
  }, [bondOffers.list])

  return null;
}

export default Updater;
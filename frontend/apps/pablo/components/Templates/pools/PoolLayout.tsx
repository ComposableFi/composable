import { FC, ReactNode, useEffect } from "react";
import Default from "../Default";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { subscribePools } from "@/store/pools/subscribePools";
import { subscribePoolAmount } from "@/store/pools/subscribePoolAmount";
import { subscribeOwnedLiquidity } from "@/store/pools/subscribeOwnedLiquidity";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";

export const PoolLayout: FC<{ breadcrumbs?: ReactNode[] }> = ({
  children,
  breadcrumbs,
}) => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  useEffect(() => {
    if (parachainApi) {
      const unsub1 = subscribePools(parachainApi);
      const unsub2 = subscribePoolAmount(parachainApi);
      return () => {
        unsub1();
        unsub2();
      };
    }
  }, [parachainApi]);

  useEffect(() => {
    if (selectedAccount && parachainApi) {
      console.log("Subscribe ownedLiquidity");
      return subscribeOwnedLiquidity(parachainApi, selectedAccount.address);
    }
  }, [parachainApi, selectedAccount]);

  return <Default breadcrumbs={breadcrumbs}>{children}</Default>;
};

import { PortfolioItem } from "@/stores/defi/polkadot/stakingRewards/slice";
import { useMemo } from "react";
import { Chip } from "@mui/material";
import { Falsy, formatDate } from "shared";

export const useExpiredPortfolio = (portfolio: PortfolioItem | Falsy) => {
  const isExpired = useMemo(() => {
    if (!portfolio) return false;
    const endDate = new Date(Number(portfolio?.endTimestamp.toString()));
    const now = new Date();

    return endDate.getTime() - now.getTime() < 0;
  }, [portfolio]);

  const expiredDate = useMemo(() => {
    if (!portfolio) return null;
    const endDate = new Date(Number(portfolio?.endTimestamp.toString()));
    if (isExpired) {
      return <Chip color="warning" label="No lock period" />;
    }
    return <Chip color="success" label={formatDate(endDate)} />;
  }, [isExpired, portfolio]);

  return {
    isExpired,
    expiredDate,
  };
};

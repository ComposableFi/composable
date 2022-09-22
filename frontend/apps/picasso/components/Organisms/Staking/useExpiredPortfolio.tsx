import { PortfolioItem } from "@/stores/defi/polkadot/stakingRewards/slice";
import { useMemo } from "react";
import { Typography } from "@mui/material";
import { Falsy, formatDate } from "shared";

export const useExpiredPortfolio = (portfolio: PortfolioItem | Falsy) => {
  const isExpired = useMemo(() => {
    if (!portfolio) return false;
    const endDate = new Date(Number(portfolio?.endTimestamp.toString()));
    const now = new Date();

    return endDate.getTime() - now.getTime() < 0;
  }, [portfolio]);

  const portfolioDate = useMemo(() => {
    if (!portfolio) return null;
    const endDate = new Date(Number(portfolio?.endTimestamp.toString()));
    if (isExpired) {
      return (
        <Typography color="error" variant="body2">
          Expired
        </Typography>
      );
    }
    return <Typography variant="body2">{formatDate(endDate)}</Typography>;
  }, [isExpired, portfolio]);

  return {
    isExpired,
    portfolioDate,
  };
};

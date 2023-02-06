import { PortfolioItem } from "@/stores/defi/polkadot/stakingRewards/slice";
import { Chip } from "@mui/material";
import { humanDateDiff } from "shared";
import { useExpiredPortfolio } from "@/components/Organisms/Staking/useExpiredPortfolio";

export const StakeRemainingRelativeDate = ({
  portfolio,
}: {
  portfolio?: PortfolioItem;
}) => {
  const { isExpired } = useExpiredPortfolio(portfolio);
  const endDate = new Date(Number(portfolio?.endTimestamp.toString()));

  if (!portfolio) return null;
  if (portfolio.multiplier.eq(1) || isExpired) {
    return <Chip color="warning" label="No lock multiplier" />;
  }

  return <Chip color="success" label={humanDateDiff(endDate)} />;
};

import * as React from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableContainerProps,
  TableHead,
  TableRow,
  Typography,
} from "@mui/material";
import { BaseAsset, TokenAsset } from "../Atom";
import BigNumber from "bignumber.js";
import { NoAssetsCover } from "./NoAssetsCover";
import { TokenPairAsset } from "../Atom/TokenPairAsset";
import {
  getClaimable,
  getTokenString,
} from "@/components/Organisms/Bond/utils";
import { useCurrentBlockAndTime } from "@/defi/polkadot/utils";
import { useBlockInterval, usePicassoProvider } from "@/defi/polkadot/hooks";
import { secondsToDHMS } from "@/defi/polkadot/hooks/useBondVestingInDays";
import { ActiveBond } from "@/stores/defi/polkadot/bonds/slice";
import { fromPica } from "@/defi/polkadot/pallets/BondedFinance";

export type MyBondingsTableProps = TableContainerProps & {
  onRowClick?: (offerId: string) => void;
  openPositions: any; // TODO(Mamali): Fix type
};

export const MyBondingsTable: React.FC<MyBondingsTableProps> = ({
  openPositions,
  onRowClick = () => {},
  ...rest
}) => {
  const { parachainApi } = usePicassoProvider();
  const { block, time } = useCurrentBlockAndTime(parachainApi);
  const interval = useBlockInterval();

  if (openPositions.length > 0) {
    return (
      <TableContainer {...rest}>
        <Table sx={{ minWidth: 420 }} aria-label="simple table">
          <TableHead>
            <TableRow>
              <TableCell align="left">Asset</TableCell>
              <TableCell align="left">Claimable</TableCell>
              <TableCell align="left">Pending</TableCell>
              <TableCell align="left">Vesting Time</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {openPositions.map(
              ({ window, periodCount, perPeriod, bond }: ActiveBond) => {
                const lastBlock = window.blockNumberBased.start
                  .plus(window.blockNumberBased.period)
                  .multipliedBy(periodCount);
                const claimable = getClaimable(
                  block,
                  window,
                  perPeriod,
                  lastBlock,
                  periodCount
                );

                const total = periodCount.multipliedBy(fromPica(perPeriod));

                const pending = total.minus(claimable);
                const remainingBlocks = lastBlock.minus(block).lte(0)
                  ? new BigNumber(0)
                  : lastBlock.minus(block);
                const remainingTime = secondsToDHMS(
                  remainingBlocks
                    .multipliedBy(Number(interval) / 1000)
                    .toNumber()
                );
                const vesting_time = `${remainingTime.d
                  .toString()
                  .padStart(2, "00")}D${remainingTime.h
                  .toString()
                  .padStart(2, "00")}H${remainingTime.m
                  .toString()
                  .padStart(2, "00")}M${remainingTime.s
                  .toString()
                  .padStart(2, "00")}S`;
                return (
                  <TableRow
                    sx={{
                      "&:hover": {
                        cursor: "pointer",
                      },
                    }}
                    key={getTokenString(bond.reward.asset)}
                    onClick={() => onRowClick(bond.bondOfferId)}
                  >
                    <TableCell align="left">
                      {Array.isArray(bond.asset) && (
                        <TokenPairAsset
                          tokenIds={bond.asset.map(({ id }) => id)}
                        />
                      )}
                      {!Array.isArray(bond.asset) && (
                        <TokenAsset tokenId={bond.asset.id} />
                      )}
                    </TableCell>
                    <TableCell align="left">
                      <BaseAsset
                        icon="/tokens/chaos.svg"
                        label={`${new BigNumber(claimable).toFormat()} ${
                          Array.isArray(bond.reward.asset)
                            ? bond.reward.asset[0].id
                            : bond.reward.asset.id
                        }`}
                      />
                    </TableCell>
                    <TableCell align="left">
                      <BaseAsset
                        icon="/tokens/chaos.svg"
                        label={`${new BigNumber(pending).toFormat()} Chaos`}
                      />
                    </TableCell>
                    <TableCell align="left">
                      <Typography variant="body2">{vesting_time}</Typography>
                    </TableCell>
                  </TableRow>
                );
              }
            )}
          </TableBody>
        </Table>
      </TableContainer>
    );
  } else {
    return <NoAssetsCover />;
  }
};

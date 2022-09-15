import { FC, useEffect, useState } from "react";
import {
  Alert,
  Box,
  Paper,
  Stack,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Typography,
  useTheme
} from "@mui/material";
import { TokenAsset } from "@/components";
import { useQuery } from "@apollo/client";
import { GET_STAKING_POSITIONS, StakingPositions } from "@/apollo/queries/stakingPositions";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { callbackGate, formatDate, fromChainIdUnit, humanBalance, unwrapNumberOrHex } from "shared";
import { StakingPortfolioLoadingState } from "@/components/Organisms/Staking/StakingPortfolioLoadingState";
import BigNumber from "bignumber.js";


export const StakingPortfolio: FC = () => {
  const theme = useTheme();
  const account = useSelectedAccount();
  const { parachainApi } = usePicassoProvider();
  const [multiplierMap, setMultiplierMap] = useState<{
    [key: string]: {
      [key: string]: {
        share: BigNumber;
        stake: BigNumber;
        multiplier: BigNumber;
      }
    }
  }>({});
  const { data, loading, error } = useQuery<StakingPositions>(GET_STAKING_POSITIONS, {
    variables: {
      accountId: account?.address
    },
    pollInterval: 30000
  });

  useEffect(() => {
    const stakingPositions = data?.stakingPositions;
    callbackGate(async (positions, api) => {
      if (loading) return;
      let map: any = {};
      for (const position of positions) {
        try {
          const result: any = (await api.query.stakingRewards.stakes(
            api.createType("u128", position.fnftCollectionId),
            api.createType("u64", position.fnftInstanceId)
          )).toJSON();
          map = {
            ...map,
            [position.fnftCollectionId]: {
              ...map[position.fnftCollectionId],
              [position.fnftInstanceId]: {
                multiplier: unwrapNumberOrHex(result.share).div(
                  unwrapNumberOrHex(result.stake)),
                share: unwrapNumberOrHex(result.share),
                stake: unwrapNumberOrHex(result.stake)
              }
            }
          };
        } catch (error) {
          console.log(error);
        }
      }

      setMultiplierMap(map);
    }, stakingPositions, parachainApi);
  }, [data?.stakingPositions, loading, setMultiplierMap]);
  if (!data || loading) {
    return <StakingPortfolioLoadingState />;
  }

  return (
    <Paper sx={{ padding: theme.spacing(6), marginTop: theme.spacing(9) }}>
      <Stack gap={6}>
        <Typography variant="h6">Portfolio</Typography>
        {Object.values(multiplierMap).length > 0 ? (
          <TableContainer component={Box}>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>fNFTID</TableCell>
                  <TableCell>Locked PICA</TableCell>
                  <TableCell>Expiry Date</TableCell>
                  <TableCell>Multiplier</TableCell>
                  <TableCell>Your CHAOS</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {data.stakingPositions.map((stakingPosition, key: number) => (
                  <TableRow key={key}>
                    <TableCell>
                      <TokenAsset tokenId={"pica"} label={`fNFT ${stakingPosition.fnftInstanceId}`} />
                    </TableCell>
                    <TableCell>{humanBalance(fromChainIdUnit(stakingPosition.amount))}</TableCell>
                    <TableCell>
                      {formatDate(new Date(Number(stakingPosition.endTimestamp.toString())))}
                    </TableCell>
                    <TableCell>{multiplierMap[stakingPosition.fnftCollectionId][stakingPosition.fnftInstanceId].multiplier.toFixed()}x</TableCell>
                    <TableCell>{fromChainIdUnit(multiplierMap[stakingPosition.fnftCollectionId][stakingPosition.fnftInstanceId].share).toFixed()}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>
        ) : (
          <>
            <Alert color="info">
              No position found.
            </Alert>
          </>
        )}
      </Stack>
    </Paper>
  );
};

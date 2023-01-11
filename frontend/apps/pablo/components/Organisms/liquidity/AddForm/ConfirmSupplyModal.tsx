import React, { FC, useEffect, useState } from "react";
import { Modal, ModalProps } from "@/components/Molecules";
import { BaseAsset, Label } from "@/components/Atoms";
import {
  alpha,
  Box,
  Button,
  IconButton,
  Typography,
  useTheme,
} from "@mui/material";
import CloseIcon from "@mui/icons-material/Close";
import BigNumber from "bignumber.js";
import {
  useExecutor,
  useParachainApi,
  useSelectedAccount,
  useSigner,
} from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import { setUiState } from "@/store/ui/ui.slice";
import { PoolConfig } from "@/store/createPool/types";
import { InputConfig } from "@/components/Organisms/liquidity/AddForm/types";
import { useAddLiquidity } from "@/defi/hooks";
import useStore from "@/store/useStore";
import { getStats, GetStatsReturn } from "@/defi/utils";
import { usePoolSpotPrice } from "@/defi/hooks/pools/usePoolSpotPrice";

export interface SupplyModalProps {
  pool: PoolConfig;
  inputConfig: InputConfig[];
  share: BigNumber;
  expectedLP: BigNumber;
  amountOne: BigNumber;
  amountTwo: BigNumber;
}

export const ConfirmSupplyModal: FC<SupplyModalProps & ModalProps> = ({
  pool,
  inputConfig,
  expectedLP,
  amountTwo,
  amountOne,
  ...rest
}) => {
  const theme = useTheme();
  const signer = useSigner();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const executor = useExecutor();
  const poolId = pool.poolId.toString();
  const [assetOne, assetTwo] = inputConfig.map((input) => input.asset);
  const { spotPrice } = usePoolSpotPrice(pool, [assetOne, assetTwo]);
  const totalIssued = useStore((store) => store.pools.totalIssued);

  const shareOfPool = expectedLP
    .div(totalIssued[pool.poolId.toString()].plus(expectedLP))
    .multipliedBy(100);

  const onConfirmSupply = useAddLiquidity({
    selectedAccount,
    executor,
    parachainApi,
    assetOneAmount: amountOne,
    assetTwoAmount: amountTwo,
    lpReceiveAmount: expectedLP,
    poolId,
    signer,
    assetIn: pool.config.assets[0],
    assetOut: pool.config.assets[1],
  });
  const isPoolsLoaded = useStore((store) => store.pools.isLoaded);
  const [stats, setStats] = useState<GetStatsReturn>(null);
  useEffect(() => {
    if (isPoolsLoaded && pool) {
      getStats(pool).then((result) => {
        setStats(result);
      });
    }
  }, [isPoolsLoaded, pool]);

  if (stats === null) return null;
  const twoPerOne = BigNumber(1).div(spotPrice).isFinite()
    ? BigNumber(1).div(spotPrice).toFixed(4)
    : "0.0000";
  const onePerTwo =
    spotPrice.isFinite() && !spotPrice.isNaN()
      ? spotPrice.toFormat(4)
      : "0.0000";

  return (
    <Modal
      onClose={() => setUiState({ isConfirmSupplyModalOpen: false })}
      {...rest}
    >
      <Box
        sx={{
          background: theme.palette.gradient.secondary,
          width: 550,
          [theme.breakpoints.down("sm")]: {
            width: "100%",
          },
          borderRadius: 1,
          padding: theme.spacing(3),
          boxShadow: `-1px -1px ${alpha(
            theme.palette.common.white,
            theme.custom.opacity.light
          )}`,
        }}
      >
        <Box display="flex" alignItems="center" justifyContent="space-between">
          <Typography variant="body1">You will receive</Typography>
          <IconButton
            onClick={() => setUiState({ isConfirmSupplyModalOpen: false })}
          >
            <CloseIcon />
          </IconButton>
        </Box>

        <Typography variant="h5" mt={1.75}>
          {expectedLP.toString()}
        </Typography>

        <Typography variant="body1" color="text.secondary" mt={1.75}>
          {`LP ${assetOne.getSymbol()}/${assetTwo.getSymbol()} Tokens`}
        </Typography>

        <Typography variant="body2" mt={4} textAlign="center" paddingX={4.25}>
          Output is estimated. If the price changes by more than 5% your
          transaction will revert.
        </Typography>

        <Box
          mt={4}
          borderTop={`1px solid ${alpha(
            theme.palette.common.white,
            theme.custom.opacity.main
          )}`}
        />

        <Label
          mt={4}
          label={`Pooled ${assetOne.getSymbol()}`}
          BalanceProps={{
            title: <BaseAsset icon={assetOne.getIconUrl()} pr={1} />,
            balance: `${amountOne.toFormat(4)}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label={`Pooled ${assetTwo.getSymbol()}`}
          BalanceProps={{
            title: <BaseAsset icon={assetTwo.getIconUrl()} pr={1} />,
            balance: `${amountTwo.toFormat(4, 1)}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label={`Price`}
          BalanceProps={{
            balance: `1 ${assetOne?.getSymbol()} = ${twoPerOne} ${assetTwo?.getSymbol()}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label=""
          BalanceProps={{
            balance: `1 ${assetTwo?.getSymbol()} = ${onePerTwo} ${assetOne?.getSymbol()}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label={`Share of pool`}
          BalanceProps={{
            balance: `${shareOfPool.toFixed(4)}%`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Box mt={4}>
          <Button
            variant="contained"
            size="large"
            fullWidth
            onClick={onConfirmSupply}
          >
            Confirm supply
          </Button>
        </Box>
      </Box>
    </Modal>
  );
};

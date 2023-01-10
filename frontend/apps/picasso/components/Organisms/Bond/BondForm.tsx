import { FC, useEffect, useState } from "react";
import {
  Box,
  Button,
  Grid,
  InputAdornment,
  Stack,
  Typography,
} from "@mui/material";
import { BigNumberInput, Modal, TokenAsset } from "@/components";
import { PairAsset } from "@/components/Atom/PairAsset";
import PositionDetailsRow from "@/components/Atom/PositionDetailsRow";
import WarningAmberRoundedIcon from "@mui/icons-material/WarningAmberRounded";
import PositionDetails from "@/components/Atom/PositionDetails";
import BigNumber from "bignumber.js";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { useExecutor, usePicassoProvider, useSigner } from "substrate-react";
import { usePicassoAccount } from "@/defi/polkadot/hooks";
import { useSnackbar } from "notistack";
import {
  getTokenString,
  lpToSymbolPair,
} from "@/components/Organisms/Bond/utils";
import { alpha, useTheme } from "@mui/material/styles";
import { getROI, purchaseBond } from "@/defi/polkadot/pallets/BondedFinance";
import { humanBalance } from "shared";

type BondOfferBalances = {
  [key: string]: BigNumber;
};
type PositionItem = {
  [key: string]: {
    label: string;
    description: string;
  };
};

export const BondForm: FC<{
  hasClaim: boolean;
  standardPageSize: { [key: string]: string | number };
  maxPurchasableBonds: BigNumber;
  bondOffer: BondOffer;
  roi: BigNumber;
  offerId: string;
  balances: BondOfferBalances;
  tokenSymbol: string;
  isLoadingBalances: boolean;
}> = ({
  hasClaim,
  standardPageSize,
  roi,
  maxPurchasableBonds,
  bondOffer,
  offerId,
  balances,
  tokenSymbol,
  isLoadingBalances,
}) => {
  const theme = useTheme();
  const [isBondValid, setBondValidation] = useState<boolean>(false);
  const [bondInput, setBondInput] = useState<BigNumber>(new BigNumber(0));
  const [open, setOpen] = useState<boolean>(false);
  const [open2nd, setOpen2nd] = useState<boolean>(false);
  const [transferDetails, setTransferDetails] = useState<PositionItem>({});
  const { parachainApi } = usePicassoProvider();
  const account = usePicassoAccount();
  const executor = useExecutor();
  const { enqueueSnackbar } = useSnackbar();
  const signer = useSigner();

  const handleApprove = () => {
    setOpen(true);
    // Approve logic here
  };

  const handleBond = () => {
    setOpen(true);
  };

  const handleFormReset = () => {
    setBondInput(new BigNumber(0));
  };

  const handlePurchase = async () => {
    if (roi.lt(0)) {
      setOpen2nd(true);
      return;
    }
    await purchaseBond({
      parachainApi,
      account,
      executor,
      signer,
      offerId,
      bondInput,
      enqueueSnackbar,
      setOpen,
      setOpen2nd,
      handleFormReset,
    });
  };

  const handleWait = () => {
    setOpen(false);
    setOpen2nd(false);
  };

  const handleBurnMoney = async () => {
    await purchaseBond({
      parachainApi,
      account,
      executor,
      offerId,
      bondInput,
      enqueueSnackbar,
      setOpen,
      setOpen2nd,
      handleFormReset,
      signer,
    });
  };

  useEffect(() => {
    if (!isLoadingBalances) {
      setTransferDetails((details) => {
        return {
          ...details,
          ...{
            yourBalance: {
              label: "Your balance",
              description: `${humanBalance(
                balances[bondOffer.assetId]
              )} ${getTokenString(bondOffer.asset)}`,
            },
          },
          ...{
            youWillGet: {
              label: "You will get",
              description: `${humanBalance(
                bondOffer.reward.amount
                  .dividedBy(bondOffer.nbOfBonds)
                  .multipliedBy(bondInput)
              )} ${getTokenString(bondOffer.reward.asset)}`,
            },
          },
          ...{
            maxYouCanBuy: {
              label: "Max you can buy",
              description: `${humanBalance(
                bondOffer.reward.amount
                  .dividedBy(bondOffer.nbOfBonds)
                  .multipliedBy(maxPurchasableBonds)
              )} ${getTokenString(bondOffer.reward.asset)}`,
            },
          },
          ...{
            roi: {
              label: "ROI",
              description: `${humanBalance(
                getROI(bondOffer.rewardPrice, bondOffer.price)
              )}%`,
            },
          },
        };
      });
    }
  }, [isLoadingBalances, bondInput]); // eslint-disable-line react-hooks/exhaustive-deps

  const DetailWrapperComponent: FC<{ hasClaim: boolean }> = ({
    hasClaim,
    children,
  }) => {
    if (hasClaim) {
      return (
        <Box
          sx={{
            marginTop: "2rem",
            width: "100%",
          }}
        >
          {children}
        </Box>
      );
    }
    return <PositionDetails>{children}</PositionDetails>;
  };

  const WrapperComponent: FC<{ hasClaim: boolean }> = ({
    hasClaim,
    children,
  }) => {
    if (hasClaim) {
      return (
        <Box
          sx={{
            flexDirection: "column",
            display: "flex",
            alignItems: "center",
            padding: "3rem",
            backgroundColor: alpha(theme.palette.common.white, 0.02),
            borderRadius: "0.75rem",
            minWidth: "50%",
            width: "50%",
          }}
        >
          {children}
        </Box>
      );
    }
    return (
      <Grid container sx={{ xs: 12 }}>
        <Grid
          item
          mt="4.5rem"
          gap={4}
          display="flex"
          flexDirection="column"
          {...standardPageSize}
        >
          {children}
        </Grid>
      </Grid>
    );
  };

  return (
    <>
      <WrapperComponent hasClaim={hasClaim}>
        <Typography
          variant="h5"
          color="text.common.white"
          textAlign="center"
          mb="2rem"
        >
          Bond
        </Typography>
        <BigNumberInput
          value={bondInput}
          isValid={(v) => setBondValidation(v)}
          setter={setBondInput}
          maxValue={maxPurchasableBonds}
          LabelProps={{
            mainLabelProps: { label: "Amount" },
            balanceLabelProps: {
              label: "Balance:",
              balanceText: `${humanBalance(balances[bondOffer.assetId] || 0)} ${
                Array.isArray(bondOffer.asset)
                  ? bondOffer.asset.reduce(lpToSymbolPair, "")
                  : bondOffer.asset.symbol
              }`,
            },
          }}
          InputProps={{
            startAdornment: (
              <InputAdornment position={"start"}>
                {Array.isArray(bondOffer.asset) ? (
                  <PairAsset assets={bondOffer.asset} />
                ) : (
                  <TokenAsset tokenId={bondOffer.asset.id} />
                )}
              </InputAdornment>
            ),
            endAdornment: (
              <InputAdornment position="end">
                <Button
                  variant="text"
                  color="primary"
                  onClick={() => setBondInput(maxPurchasableBonds)}
                >
                  Max
                </Button>
              </InputAdornment>
            ),
          }}
        />
        {/** If Bond Is negative, show approve */}
        {roi.lt(0) ? (
          <Button variant="contained" fullWidth onClick={handleApprove}>
            Approve
          </Button>
        ) : (
          <Button
            fullWidth
            variant="contained"
            onClick={handleBond}
            disabled={!isBondValid}
            sx={{
              ...(hasClaim ? { marginTop: "2rem" } : {}),
              ...(hasClaim ? { marginTop: "2rem" } : {}),
            }}
          >
            Bond
          </Button>
        )}
        {isBondValid && (
          <DetailWrapperComponent hasClaim={hasClaim}>
            {Object.values(transferDetails).map(({ label, description }) => (
              <PositionDetailsRow
                key={label}
                label={label}
                description={description}
              />
            ))}
          </DetailWrapperComponent>
        )}
      </WrapperComponent>
      <Modal open={open} onClose={() => setOpen(false)} dismissible>
        <Typography textAlign="center" variant="h6">
          Purchase Bond
        </Typography>
        {roi.lt(0) && (
          <Typography
            textAlign="center"
            variant="subtitle2"
            color="text.secondary"
            mt={theme.spacing(2)}
          >
            Are you sure you want to bond for a negative discount? <br />
            You will lose money if you do this...
          </Typography>
        )}
        <Stack mt="4rem">
          {[
            {
              label: "Bonding",
              description: `${bondInput.toFormat(4)} ${tokenSymbol}`,
            },
            {
              label: "You will get",
              description: transferDetails.youWillGet?.description,
            },
            {
              label: "Bond price",
              description: `$${bondOffer.price.toFormat(2)}`,
            },
            {
              label: "Market price",
              description: `$${bondOffer.rewardPrice.toFormat(2)}`,
            },
            {
              label: "Discount",
              description: `${humanBalance(roi)}%`,
              discountColor: roi.toNumber(),
            },
          ].map(({ label, description, discountColor }) => (
            <PositionDetailsRow
              key={label}
              label={label}
              description={description}
              descriptionColor={discountColor}
            />
          ))}
        </Stack>
        <Button
          sx={{
            mt: theme.spacing(4),
          }}
          variant="contained"
          fullWidth
          onClick={handlePurchase}
        >
          Purchase Bond
        </Button>
        <Button
          sx={{
            mt: theme.spacing(4),
          }}
          variant="text"
          fullWidth
          onClick={() => setOpen(false)}
        >
          Cancel Bond
        </Button>
      </Modal>
      {/** Second confirmation */}
      <Modal
        open={open2nd}
        onClose={() => {
          setOpen(false);
          setOpen2nd(false);
        }}
        dismissible
      >
        <Box textAlign="center" mb={theme.spacing(6)}>
          <WarningAmberRoundedIcon
            sx={{
              color: "text.secondary",
              width: 80,
              height: 80,
            }}
          />
        </Box>
        <Typography textAlign="center" variant="h6">
          Warning
        </Typography>
        <Typography
          textAlign="center"
          variant="subtitle2"
          color="text.secondary"
          mt={theme.spacing(2)}
        >
          This bond is offered at a negative discount. <br />
          Please consider waiting until bond returns to positive discount.
        </Typography>
        <Button
          sx={{
            mt: theme.spacing(8),
          }}
          variant="contained"
          fullWidth
          onClick={handleWait}
        >
          {"Ok, I'll wait"}
        </Button>
        <Button
          sx={{
            mt: theme.spacing(4),
          }}
          variant="text"
          fullWidth
          onClick={handleBurnMoney}
        >
          I want to burn money
        </Button>
      </Modal>
    </>
  );
};

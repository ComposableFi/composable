import { useEffect, useState } from "react";
import { NextPage } from "next";
import { useRouter } from "next/router";
import {
  Box,
  Button,
  Grid,
  InputAdornment,
  Skeleton,
  Stack,
  Typography,
  useTheme,
} from "@mui/material";
import WarningAmberRoundedIcon from "@mui/icons-material/WarningAmberRounded";

import { BigNumberInput, BondBox, Input, TokenAsset } from "@/components";
import Default from "@/components/Templates/Default";
import { Modal, PageTitle } from "@/components/Molecules";
import PositionDetails from "@/components/Atom/PositionDetails";
import PositionDetailsRow from "@/components/Atom/PositionDetailsRow";
import {
  balance,
  bondPrice,
  discount,
  marketPrice,
  maxToBuy,
  reward,
  roi,
  vestingPeriod,
} from "@/stores/defi/stats/dummyData";
import { useAppSelector } from "@/hooks/store";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { Updater } from "@/stores/defi/polkadot/bonds/PolkadotBondsUpdater";
import { fromPica, getROI } from "@/defi/polkadot/pallets/BondedFinance";
import { Token } from "@/defi/Tokens";
import { fetchBalanceByAssetId } from "@/defi/polkadot/pallets/Balance";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import BigNumber from "bignumber.js";
import { bool } from "@polkadot/types-codec";
import { PairAsset } from "@/components/Atom/PairAsset";
import { getSigner, useExecutor, useExtrinsics } from "substrate-react";
import { useExtrinsicCalls } from "substrate-react/src/extrinsics/hooks";
import { APP_NAME } from "@/defi/polkadot/constants";
import { useSnackbar } from "notistack";

const standardPageSize = {
  xs: 12,
};

type PositionIndex = 0 | 1 | 2 | 3 | 4;

type PositionData = {
  label: string;
  description: string;
};

type PositionItem = {
  [key: string]: PositionData;
};

type BoxPosition = 0 | 1 | 2 | 3;

type BoxData = {
  title: string;
  description: string;
  discountColor?: number;
};

type BoxItem = {
  [key in BoxPosition]: BoxData;
};

const confirmationRows = [
  {
    label: "Bonding",
    description: `${balance} LP`,
  },
  {
    label: "You will get",
    description: `${reward} CHAOS`,
  },
  {
    label: "Bond price",
    description: `$${bondPrice}`,
  },
  {
    label: "Market price",
    description: `$${marketPrice}`,
  },
  {
    label: "Discount",
    description: `${discount}%`,
    discountColor: discount,
  },
];

function lpToSymbolPair(acc: string, token: Token) {
  return acc.length > 0 ? acc + "-" + token.symbol : token.symbol;
}

function useBalanceForOffer(offer: BondOffer) {
  const { parachainApi, chainId } = usePicassoProvider();
  const account = useSelectedAccount();
  const [balances, setBalances] = useState<{
    [key: typeof offer.assetId]: BigNumber;
  }>({});

  useEffect(() => {
    if (account && parachainApi && offer) {
      fetchBalanceByAssetId(parachainApi, account.address, offer.assetId).then(
        (result) => {
          setBalances((balances) => ({
            ...balances,
            ...{ [offer.assetId]: result },
          }));
        }
      );
    }
  }, [parachainApi, account, offer]);

  return {
    balances,
    isLoading: Object.keys(balances).length === 0,
  };
}

function getMaxPurchasableBonds(bondOffer: BondOffer, balance: BigNumber) {
  if (!bondOffer || !balance) return new BigNumber(0);
  const maxBonds = bondOffer.price.multipliedBy(bondOffer.nbOfBonds);
  const purchasableBonds = balance.modulo(maxBonds).absoluteValue();

  if (purchasableBonds.lt(1)) {
    return purchasableBonds.absoluteValue();
  } else if (purchasableBonds.gte(maxBonds)) {
    return maxBonds;
  }

  return purchasableBonds;
}

const Bond: NextPage = () => {
  const router = useRouter();
  const { bond } = router.query;
  const bondOffer = useAppSelector<BondOffer>(
    (state) => state.bonding.bonds[Number(bond) - 1]
  );
  const theme = useTheme();
  const [open, setOpen] = useState<boolean>(false);
  const [open2nd, setOpen2nd] = useState<boolean>(false);
  const { parachainApi, chainId } = usePicassoProvider();
  const account = useSelectedAccount();
  const [transferDetails, setTransferDetails] = useState<PositionItem>({});
  const { isLoading: isLoadingBalances, balances } =
    useBalanceForOffer(bondOffer);
  const executor = useExecutor();
  const { enqueueSnackbar } = useSnackbar();

  const maxPurchasableBond = getMaxPurchasableBonds(
    bondOffer,
    balances[bondOffer?.assetId]
  );
  /*
   * Populating transfer details
   */
  useEffect(() => {
    if (!isLoadingBalances) {
      setTransferDetails((transferDetails) => {
        return {
          ...transferDetails,
          ...{
            yourBalance: {
              label: "Your balance",
              description: `${balances[bondOffer.assetId].toFormat(0)} ${
                Array.isArray(bondOffer.asset)
                  ? bondOffer.asset.reduce(lpToSymbolPair, "")
                  : bondOffer.asset.symbol
              }`,
            },
          },
          ...{
            youWillGet: {
              label: "You will get",
              description: `${balances[bondOffer.assetId].toFormat(0)} ${
                Array.isArray(bondOffer.reward.asset)
                  ? bondOffer.reward.asset.reduce(lpToSymbolPair, "")
                  : bondOffer.reward.asset.symbol
              }`,
            },
          },
          ...{
            maxYouCanBuy: {
              label: "Max you can buy",
              description: `${maxPurchasableBond
                .multipliedBy(bondOffer.reward.amount)
                .toFormat(0)}`,
            },
          },
          ...{
            roi: {
              label: "ROI",
              description: `${getROI(
                bondOffer.rewardPrice,
                bondOffer.price
              ).toFixed(3)}%`,
            },
          },
        };
      });
    }
  }, [isLoadingBalances]);

  /*
   * Form related
   */
  const [isBondValid, setBondValidation] = useState<boolean>(false);
  const [bondInput, setBondInput] = useState<BigNumber>(new BigNumber(0));

  if (!bondOffer || !bond) {
    return (
      <Default>
        <Updater />
        <Box
          display={"flex"}
          width="100%"
          alignItems="center"
          justifyContent="center"
        >
          <Grid
            container
            maxWidth={1032}
            display="flex"
            justifyContent="center"
            gap={9}
            mt={9}
          >
            <Grid item width="100%" display="flex" justifyContent="center">
              <Skeleton variant="text" width={270} height={111} />
            </Grid>
            <Grid
              item
              display="flex"
              justifyContent="space-between"
              width="100%"
            >
              <Skeleton
                variant="rectangular"
                width={234}
                height={118}
                sx={{ borderRadius: `${theme.shape.borderRadius}px` }}
              />
              <Skeleton
                variant="rectangular"
                width={234}
                height={118}
                sx={{ borderRadius: `${theme.shape.borderRadius}px` }}
              />
              <Skeleton
                variant="rectangular"
                width={234}
                height={118}
                sx={{ borderRadius: `${theme.shape.borderRadius}px` }}
              />
              <Skeleton
                variant="rectangular"
                width={234}
                height={118}
                sx={{ borderRadius: `${theme.shape.borderRadius}px` }}
              />
            </Grid>
            <Grid item width="100%" display="flex" justifyContent="center">
              <Skeleton variant="text" width={279} height={118} />
            </Grid>
            <Grid item width="100%" display="flex" justifyContent="center">
              <Skeleton variant="text" width={1032} height={118} />
            </Grid>
            <Grid item width="100%" display="flex" justifyContent="center">
              <Skeleton variant="text" width={1032} height={118} />
            </Grid>
          </Grid>
        </Box>
      </Default>
    );
  }

  const token = Array.isArray(bondOffer.asset)
    ? bondOffer.asset.reduce(lpToSymbolPair, "")
    : bondOffer.asset.symbol;
  const toToken = Array.isArray(bondOffer.reward.asset)
    ? bondOffer.reward.asset.reduce(lpToSymbolPair, "")
    : bondOffer.reward.asset.symbol;

  const roi = getROI(bondOffer.rewardPrice, bondOffer.price);

  const bondBoxes: BoxItem = {
    0: {
      title: "Bond price",
      description: `$${bondOffer.price.toFormat(0)}`,
    },
    1: {
      title: "Market price",
      description: `$${bondOffer.rewardPrice.toFormat(0)}`,
    },
    2: {
      title: "Discount",
      description: `${roi.toFixed(3)}%`,
      discountColor: Number(roi.toFixed(3)),
    },
    3: {
      title: "Vesting period",
      description: `${vestingPeriod} days`,
    },
  };

  const handleApprove = () => {
    setOpen(true);
    // Approve logic here
  };

  const handleBond = () => {
    setOpen(true);
  };

  const handlePurchase = async () => {
    if (roi.lt(0)) {
      setOpen2nd(true);
      return;
    }
    console.log("Purchasing...");
    // bond(offerId, nbOfBonds, keepAlive);
    if (parachainApi && account && executor) {
      console.log("parachain exist...");
      try {
        const signer = await getSigner(APP_NAME, account.address);
        await executor
          .execute(
            parachainApi.tx.bondedFinance.bond(
              bond.toString(),
              bondInput.toString(),
              true
            ),
            account.address,
            parachainApi,
            signer,
            (txHash: string) => {
              enqueueSnackbar("Initiating Transaction");
              console.log("Bail MOdal here")
            },
            (txHash: string, events) => {
              enqueueSnackbar("Transaction Finalized");
              console.log('SOmething else here')
            }
          )
          .catch((err) => {
            enqueueSnackbar(err.message);
          });
        console.log("Whats happening here");
      } catch (e) {
        console.log(e);
      }
    } else {
      console.log("Purchasing... no parachainAPI");
    }
  };

  const handleWait = () => {
    setOpen(false);
    setOpen2nd(false);
  };

  const handleBurnMoney = () => {
    setOpen(false);
    setOpen2nd(false);
    // Purchase with negative discount here
  };

  return (
    <Default>
      <Updater />
      <Box
        flexGrow={1}
        sx={{ mx: "auto" }}
        maxWidth={1032}
        mt={theme.spacing(9)}
      >
        <Grid container alignItems="center" gap={theme.spacing(9)}>
          <Grid item {...standardPageSize}>
            <PageTitle
              title={`${token}-${toToken}`}
              subtitle="Purchase CHAOS at a discount"
              textAlign="center"
            />
          </Grid>
          <Grid item container spacing={3}>
            {Object.values(bondBoxes).map(
              ({ title, description, discountColor }) => (
                <Grid item key={title} xs={3}>
                  <BondBox
                    title={title}
                    description={description}
                    discountColor={discountColor}
                  />
                </Grid>
              )
            )}
          </Grid>
          <Grid
            item
            {...standardPageSize}
            mt="4.5rem"
            gap={4}
            display="flex"
            flexDirection="column"
          >
            <Typography
              variant="h5"
              color="text.common.white"
              textAlign="center"
              mb="3.813rem"
            >
              Bond
            </Typography>
            <BigNumberInput
              value={bondInput}
              isValid={(v) => setBondValidation(v)}
              setter={setBondInput}
              maxValue={maxPurchasableBond}
              LabelProps={{
                mainLabelProps: { label: "Amount" },
                balanceLabelProps: {
                  label: "Balance:",
                  balanceText: `${balances[bondOffer.assetId]?.toFormat(0)} ${
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
                      <TokenAsset tokenId={bondOffer.asset.symbol} />
                    )}
                  </InputAdornment>
                ),
                endAdornment: (
                  <InputAdornment position="end">
                    <Button
                      variant="text"
                      color="primary"
                      onClick={() => setBondInput(maxPurchasableBond)}
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
              <Button fullWidth variant="contained" onClick={handleBond}>
                Bond
              </Button>
            )}

            {/** First confirmation */}
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
                    description: `${bondInput.toFormat(4)} ${token}`,
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
                    description: `${roi.toFormat(3)}%`,
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
                This bond is bondOfferly at a negative discount. <br />
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
          </Grid>
          <Grid item {...standardPageSize}>
            <PositionDetails>
              {Object.values(transferDetails).map(({ label, description }) => (
                <PositionDetailsRow
                  key={label}
                  label={label}
                  description={description}
                />
              ))}
            </PositionDetails>
          </Grid>
        </Grid>
      </Box>
    </Default>
  );
};

export default Bond;

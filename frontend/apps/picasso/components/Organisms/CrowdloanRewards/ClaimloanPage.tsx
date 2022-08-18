import {
  AlertBox,
  DefaultLayout,
  KSMClaimForm,
  Link,
  NoEligibleWalletFeaturedBox,
  PageTitle,
  SS8WalletHelper,
  StablecoinClaimForm,
} from "@/components";
import { ParachainContext } from "@/defi/polkadot/context/ParachainContext";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { useStore } from "@/stores/root";
import { alpha, Grid, Typography, useTheme } from "@mui/material";
import { ApiPromise } from "@polkadot/api";
import { stringToHex } from "@polkadot/util";
import { useContext, useEffect, useState } from "react";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { crowdLoanSignableMessage } from "@/utils/crowdloanRewards";
import { toBaseUnitBN, toTokenUnitsBN } from "shared";
import { useRouter } from "next/router";
import { ConnectorType, useBlockchainProvider, useConnector } from "bi-lib";
import { updateBalances } from "@/stores/defi/polkadot/balances/PolkadotBalancesUpdater";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { OpenInNewRounded } from "@mui/icons-material";
import CheckCircleOutlineIcon from "@mui/icons-material/CheckCircleOutline";
import { useExecutor, usePendingExtrinsic } from "substrate-react";
import { useSnackbar } from "notistack";
import BigNumber from "bignumber.js";

const DEFAULT_EVM_ID = 1;
const PICA_CHAIN_ID = "kusama-2019";
const APP_NAME = "Picasso UI";

const ERROR_MESSAGES = {
  KSM_WALLET_NOT_CONNECTED: {
    message:
      "Please connect the KSM address used to contribute to the Picasso crowdloan.",
    title: "Nothing to claim",
  },
  WRONG_ADDRESS: {
    message:
      "Please connect the address used to contribute to Picasso crowdloan.",
    title: "Nothing to claim",
  },
  ETH_WALLET_NOT_CONNECTED: {
    message: "Please connect metamask to claim PICA rewards.",
    title: "Nothing to claim",
  },
};

interface Claimloan {
  isStable: boolean;
}

export const ClaimloanPage = ({ isStable = false }: Claimloan) => {
  const router = useRouter();
  const { enqueueSnackbar } = useSnackbar();
  const { isActive } = useConnector(ConnectorType.MetaMask);
  const { signer } = useBlockchainProvider(DEFAULT_EVM_ID);
  const executor = useExecutor();

  const {
    setEvmAlreadyAssociated,
    setUseAssociationMode,
    setUserAssociatedWith,
    setUserClaimEligibility,
    setUserCrowdloanData,
  } = useStore(({ crowdloanRewards }) => crowdloanRewards);
  const { updateBalance } = useStore(
    ({ substrateBalances }) => substrateBalances
  );

  const { closeKSMClaimModal, openKSMClaimModal } = useStore(({ ui }) => ui);
  const crUiState = useStore(({ crowdloanRewards }) => crowdloanRewards.ui);
  const crUserData = useStore(({ crowdloanRewards }) => crowdloanRewards.user);
  const initalPayment = useStore(
    ({ crowdloanRewards }) => crowdloanRewards.constants.initialPayment
  );
  const userAssociation = useStore(
    ({ crowdloanRewards }) => crowdloanRewards.associatedWith
  );
  const isEvmAlreadyAssociated = useStore(
    ({ crowdloanRewards }) => crowdloanRewards.evmAlreadyAssociated
  );
  const { extensionStatus } = useContext(ParachainContext);
  const selectedAccount = useSelectedAccount();
  const {parachainApi, accounts} = usePicassoProvider();
  const theme = useTheme();
  const [ineligibleText, setIneligibleText] = useState({
    title: ERROR_MESSAGES.KSM_WALLET_NOT_CONNECTED.title,
    textBelow: ERROR_MESSAGES.KSM_WALLET_NOT_CONNECTED.message,
  });
  const [showAlertBox, setShowAlertBox] = useState<boolean>(false);

  const hasNothingToClaim = (): boolean => {
    return (
      extensionStatus !== "connected" ||
      (crUiState.useAssociationMode === "ethereum" && !isActive) ||
      (userAssociation === null && !crUiState.isEligible)
    );
  };

  const onAssociationSuccess = (
    associationMode: "relayChain" | "ethereum",
    account: string
  ) => {
    if (associationMode === "ethereum") {
      setEvmAlreadyAssociated(true);
    }

    setUserAssociatedWith(associationMode);

    setUserClaimEligibility(true);

    closeKSMClaimModal();

    updateBalances(
      account,
      parachainApi,
      "kusama-2019" as SubstrateNetworkId,
      updateBalance
    );
  };

  const onAssociationFail = (err: any) => {
    if (
      err.message === "1010: Invalid Transaction: Custom error: 1" ||
      err.message === "1010: Invalid Transaction: Custom error: 3"
    ) {
      setUserClaimEligibility(false);
      setIneligibleText((s) => {
        s.title = ERROR_MESSAGES.WRONG_ADDRESS.title;
        s.textBelow = ERROR_MESSAGES.WRONG_ADDRESS.message;
        return { ...s };
      });
    }

    closeKSMClaimModal();
  };

  const onClaim = async () => {
    if (parachainApi && selectedAccount) {
      if (crUiState.isEligible && userAssociation === null) {
        openKSMClaimModal();

        crUiState.useAssociationMode === "relayChain"
          ? associateKSM(selectedAccount.address, parachainApi)
          : associateETH(selectedAccount.address, parachainApi);
      } else if (crUiState.isEligible && userAssociation !== null) {
        // claim
        openKSMClaimModal();
        claim(selectedAccount.address);
      }
    }
  };

  useEffect(() => {
    const pathNames = router.pathname.split("/");
    const pathName = pathNames[pathNames.length - 1];

    if (pathName.toLowerCase() === "ksm") {
      setUseAssociationMode("relayChain");
    } else {
      setUseAssociationMode("ethereum");
    }
    // Only to be called on page load therefore we can omit dependencies.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    if (crUiState.useAssociationMode === "ethereum") {
      if (!isActive) {
        setIneligibleText((s) => {
          s.textBelow = ERROR_MESSAGES.ETH_WALLET_NOT_CONNECTED.message;
          return s;
        });
      }
    }

    if (
      extensionStatus !== "connected" ||
      !accounts.length
    ) {
      setIneligibleText((s) => {
        s.textBelow = ERROR_MESSAGES.KSM_WALLET_NOT_CONNECTED.message;
        return s;
      });
    }
  }, [
    accounts,
    crUiState.useAssociationMode,
    extensionStatus,
    isActive,
    selectedAccount,
  ]);

  let netPICAVested = new BigNumber(crUserData.netVestedPICA);
  let contributedAmount = new BigNumber(crUserData.contribution);

  let claimedPICA = toTokenUnitsBN(
    crUserData.claimedPICA,
    SUBSTRATE_NETWORKS[PICA_CHAIN_ID].decimals
  );

  let claimablePICA = toTokenUnitsBN(
    crUserData.claimablePICA,
    SUBSTRATE_NETWORKS[PICA_CHAIN_ID].decimals
  );

  claimablePICA =
    Number(initalPayment) > 0 && netPICAVested.gte(0)
      ? crUiState.useAssociationMode === "ethereum"
        ? isEvmAlreadyAssociated
          ? claimablePICA
          : netPICAVested.times(initalPayment)
        : crUiState.useAssociationMode === "relayChain"
        ? userAssociation === null
          ? netPICAVested.times(initalPayment)
          : claimablePICA
        : claimablePICA
      : claimablePICA;

  const isPendingClaim = usePendingExtrinsic(
    "claim",
    "crowdloanRewards",
    selectedAccount ? selectedAccount.address : ""
  );

  const isPendingAssociate = usePendingExtrinsic(
    "associate",
    "crowdloanRewards",
    ""
  );

  const claim = async (address: string) => {
    try {
      const {
        web3FromAddress,
        web3Enable,
      } = require("@polkadot/extension-dapp");
      await web3Enable(APP_NAME);
      const injector = await web3FromAddress(address);

      if (executor && parachainApi && selectedAccount) {
        let toUpdateAmount = claimablePICA.plus(claimedPICA);
        await executor.execute(
          parachainApi.tx.crowdloanRewards.claim(),
          selectedAccount.address,
          parachainApi,
          injector.signer,
          (txHash) => {
            enqueueSnackbar("Claim Processing", {
              variant: "info",
              isClosable: true,
              url: SUBSTRATE_NETWORKS["kusama-2019"].subscanUrl + txHash,
            });
          },
          (txHash) => {
            enqueueSnackbar("Claim Finalized", {
              variant: "success",
              isClosable: true,
              url: SUBSTRATE_NETWORKS["kusama-2019"].subscanUrl + txHash,
            });

            setUserCrowdloanData(
              "0",
              netPICAVested.toString(),
              toBaseUnitBN(
                toUpdateAmount.toString(),
                SUBSTRATE_NETWORKS["kusama-2019"].decimals
              ).toString()
            );

            closeKSMClaimModal();
          }
        );
      }
    } catch (err: any) {
      console.log(err);
      closeKSMClaimModal();
    }
  };

  const associateKSM = async (address: string, api: ApiPromise) => {
    const { web3FromAddress, web3Enable } = require("@polkadot/extension-dapp");
    await web3Enable(APP_NAME);
    const injector = await web3FromAddress(address);

    if (injector.signer.signRaw && executor) {
      try {
        const accId32 = api.createType("AccountId32", address);
        const { signature } = await injector.signer.signRaw({
          address: address,
          data: stringToHex(crowdLoanSignableMessage(accId32)),
          type: "bytes",
        });

        const param = {
          RelayChain: [accId32, { Sr25519: signature }],
        };

        let toUpdateAmount = claimedPICA.plus(claimablePICA);
        await executor.executeUnsigned(
          api.tx.crowdloanRewards.associate(accId32, param),
          api,
          (_txHash) => {
            enqueueSnackbar("Claim Processing", {
              variant: "info",
              isClosable: true,
              url: SUBSTRATE_NETWORKS["kusama-2019"].subscanUrl + _txHash,
            });
          },
          (_txHash) => {
            setUserCrowdloanData(
              "0",
              netPICAVested.toString(),
              toBaseUnitBN(
                toUpdateAmount.toString(),
                SUBSTRATE_NETWORKS["kusama-2019"].decimals
              ).toString()
            );

            enqueueSnackbar("Claim Finalized", {
              variant: "info",
              isClosable: true,
              url: SUBSTRATE_NETWORKS["kusama-2019"].subscanUrl + _txHash,
            });
            onAssociationSuccess("relayChain", address);
          }
        );
      } catch (err: any) {
        onAssociationFail(err);
      }
    }
  };

  const associateETH = async (address: string, api: ApiPromise) => {
    const { web3Enable } = require("@polkadot/extension-dapp");
    await web3Enable(APP_NAME);

    if (signer && executor) {
      try {
        const accId32 = api.createType("AccountId32", address);
        const signature = await signer.signMessage(
          crowdLoanSignableMessage(accId32)
        );

        const param = {
          Ethereum: signature,
        };

        let toUpdateAmount = claimedPICA.plus(claimablePICA);
        await executor.executeUnsigned(
          api.tx.crowdloanRewards.associate(accId32, param),
          api,
          (_txHash) => {
            enqueueSnackbar("Claim Processing", {
              variant: "info",
              isClosable: true,
              url: SUBSTRATE_NETWORKS["kusama-2019"].subscanUrl + _txHash,
            });
          },
          (_txHash) => {
            setUserCrowdloanData(
              "0",
              netPICAVested.toString(),
              toBaseUnitBN(
                toUpdateAmount.toString(),
                SUBSTRATE_NETWORKS["kusama-2019"].decimals
              ).toString()
            );

            enqueueSnackbar("Claim Finalized", {
              variant: "info",
              isClosable: true,
              url: SUBSTRATE_NETWORKS["kusama-2019"].subscanUrl + _txHash,
            });
            onAssociationSuccess("ethereum", address);
          }
        );
      } catch (err: any) {
        onAssociationFail(err);
      }
    }
  };

  const breadcrumbs = [
    <Link key="Overview" underline="none" color="primary" href="/frontend/fe/apps/picasso/pages">
      Overview
    </Link>,
    <Link
      key="Crowdloan"
      underline="none"
      color="primary"
      href="/crowdloan-rewards"
    >
      Crowdloan Rewards
    </Link>,
    <Typography key="claims" color="text.secondary">
      {crUiState.useAssociationMode === "ethereum" ? "Stablecoin" : "KSM"}
    </Typography>,
  ];

  const standardPageSize = {
    xs: 12,
  };

  return (
    <DefaultLayout breadcrumbs={breadcrumbs}>
      <Grid
        container
        sx={{ mx: "auto" }}
        maxWidth={1032}
        rowSpacing={9}
        columns={10}
        direction="column"
        justifyContent="center"
        pb={9}
      >
        <Grid item {...standardPageSize} mt={theme.spacing(9)}>
          <PageTitle
            title={isStable ? "Stablecoin Contribution." : "KSM Contribution"}
            textAlign="center"
            subtitle="You will be able to check on your positions here."
          />
        </Grid>
        {hasNothingToClaim() && (
          <Grid item {...standardPageSize} mt={theme.spacing(9)}>
            <NoEligibleWalletFeaturedBox
              title={ineligibleText.title}
              textBelow={ineligibleText.textBelow}
            />
          </Grid>
        )}
        {!hasNothingToClaim() && (
          <Grid item {...standardPageSize} mt={theme.spacing(9)}>
            {isStable ? (
              <StablecoinClaimForm
                disabled={Boolean(
                  (userAssociation !== null && claimablePICA.lte(0)) ||
                    isPendingClaim ||
                    isPendingAssociate
                )}
                claimedPICA={claimedPICA}
                crowdLoanContribution={contributedAmount}
                readonlyCrowdLoanContribution={true}
                availableToClaim={claimablePICA}
                readonlyAvailableToClaim
                totalPicaVested={netPICAVested}
                readonlyTotalPicaVested
                SS8Address={selectedAccount ? selectedAccount.address : ""}
                readonlySS8Address
                onClaim={onClaim}
                onChange={(name: string, value: unknown) => {
                  console.log("Change", name, value);
                }}
              />
            ) : (
              <KSMClaimForm
                disabled={Boolean(
                  (userAssociation !== null && claimablePICA.lte(0)) ||
                    isPendingClaim ||
                    isPendingAssociate
                )}
                claimedPICA={claimedPICA}
                crowdLoanContribution={contributedAmount}
                readonlyCrowdLoanContribution={true}
                needAssociation={userAssociation === null}
                availableToClaim={claimablePICA}
                readonlyAvailableToClaim
                totalPicaVested={netPICAVested}
                readonlyTotalPicaVested
                account={selectedAccount ? selectedAccount.name : ""}
                readonlySS8Address
                onClaim={onClaim}
                onChange={(name: string, value: unknown) => {
                  console.log("Change", name, value);
                }}
                useAssociationMode={"relayChain"}
              />
            )}
          </Grid>
        )}
        <Grid item {...standardPageSize}>
          <SS8WalletHelper />
        </Grid>
        {false && showAlertBox && (
          <Grid item {...standardPageSize}>
            <AlertBox
              underlined
              icon={
                <CheckCircleOutlineIcon
                  sx={{
                    color: alpha(
                      theme.palette.text.primary,
                      theme.custom.opacity.darker
                    ),
                  }}
                />
              }
              link={
                <Link
                  key="Crowdloan"
                  underline="none"
                  color="primary"
                  href="/crowdloan-rewards"
                  target="_blank"
                >
                  <OpenInNewRounded />
                </Link>
              }
              mt={4}
              dismissible
              onClose={() => setShowAlertBox(false)}
            >
              Transaction successfull
            </AlertBox>
          </Grid>
        )}
      </Grid>
    </DefaultLayout>
  );
};

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
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { useStore } from "@/stores/root";
import { alpha, Grid, Typography, useTheme } from "@mui/material";
import { stringToHex } from "@polkadot/util";
import { useCallback, useEffect, useMemo, useState } from "react";
import { crowdLoanSignableMessage } from "@/utils/crowdloanRewards";
import { useRouter } from "next/router";
import { ConnectorType, useBlockchainProvider, useConnector } from "bi-lib";
import { OpenInNewRounded } from "@mui/icons-material";
import {
  useDotSamaContext,
  useExecutor,
  usePendingExtrinsic,
} from "substrate-react";
import { useSnackbar } from "notistack";
import CheckCircleOutlineIcon from "@mui/icons-material/CheckCircleOutline";
import {
  CrowdloanStep,
  useCrowdloanRewardsSlice,
} from "@/stores/defi/polkadot/crowdloanRewards/crowdloanRewards.slice";
import { useCrowdloanRewardsClaim } from "@/defi/polkadot/hooks/crowdloanRewards/useCrowdloanRewardsClaim";
import { useCrowdloanRewardsAssociate } from "@/defi/polkadot/hooks/crowdloanRewards/useCrowdloanRewardsAssociate";
import {
  useCrowdloanNextStep,
  useClaimableAmount,
  useClaimedAmount,
  useCrowdloanContributions,
  useEligibility,
  useEthereumAssociatedAccount,
  useHasStartedCrowdloan,
} from "@/stores/defi/polkadot/crowdloanRewards/hooks";

const DEFAULT_EVM_ID = 1;
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

interface ClaimLoan {
  isStable: boolean;
}

export const ClaimLoanPage = ({ isStable = false }: ClaimLoan) => {
  const router = useRouter();
  const { enqueueSnackbar } = useSnackbar();
  const { isActive } = useConnector(ConnectorType.MetaMask);
  const { signer, account } = useBlockchainProvider(DEFAULT_EVM_ID);
  const executor = useExecutor();

  const updateBalance = useStore(
    ({ substrateBalances }) => substrateBalances.updateBalance
  );

  const { closeKSMClaimModal, openKSMClaimModal } = useStore(({ ui }) => ui);
  const { extensionStatus } = useDotSamaContext();
  const { parachainApi, accounts } = usePicassoProvider();

  const selectedAccount = useSelectedAccount();
  const theme = useTheme();
  const [ineligibleText, setIneligibleText] = useState({
    title: ERROR_MESSAGES.KSM_WALLET_NOT_CONNECTED.title,
    textBelow: ERROR_MESSAGES.KSM_WALLET_NOT_CONNECTED.message,
  });
  const [showAlertBox, setShowAlertBox] = useState<boolean>(false);

  const flow = useMemo(() => {
    const pathNames = router.pathname.split("/");
    const pathName = pathNames[pathNames.length - 1];

    if (pathName.toLowerCase() === "ksm") {
      return "KSM";
    } else {
      return "Stable coin";
    }
  }, [router]);

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

  const signPolkadotJs = useCallback(async (): Promise<string> => {
    try {
      const {
        web3FromAddress,
        web3Enable,
      } = require("@polkadot/extension-dapp");
      await web3Enable(APP_NAME);
      if (!selectedAccount || !parachainApi)
        throw new Error("Missing Connection");
      const injector = await web3FromAddress(selectedAccount.address);
      if (!injector.signer.signRaw) throw new Error("Missing Connection");
      if (!parachainApi || !selectedAccount)
        throw new Error("Missing Connection");
      const accId32 = parachainApi.createType(
        "AccountId32",
        selectedAccount.address
      );
      const { signature } = await injector.signer.signRaw({
        address: selectedAccount.address,
        data: stringToHex(crowdLoanSignableMessage(accId32)),
        type: "bytes",
      });

      return signature;
    } catch (err: any) {
      enqueueSnackbar(err.message, { variant: "error" });
      return Promise.reject(new Error(err.message));
    }
  }, [selectedAccount, parachainApi, enqueueSnackbar]);

  const signEthereum = useCallback(async (): Promise<string> => {
    try {
      if (!parachainApi || !signer || !selectedAccount)
        throw new Error("Missing Connection");
      const accId32 = parachainApi.createType(
        "AccountId32",
        selectedAccount.address
      );
      const signature = await signer.signMessage(
        crowdLoanSignableMessage(accId32)
      );

      return signature;
    } catch (err: any) {
      enqueueSnackbar(err.message, { variant: "error" });
      return Promise.reject(new Error(err.message));
    }
  }, [selectedAccount, signer, parachainApi, enqueueSnackbar]);

  const { initialPayment } = useCrowdloanRewardsSlice();

  const { isEthAccountEligible, isPicassoAccountEligible } = useEligibility(
    account?.toLowerCase(),
    selectedAccount?.address
  );

  useEffect(() => {
    if (flow === "KSM" && extensionStatus !== "connected")
      setIneligibleText({
        title: ERROR_MESSAGES.KSM_WALLET_NOT_CONNECTED.title,
        textBelow: ERROR_MESSAGES.KSM_WALLET_NOT_CONNECTED.message,
      });

    if (flow === "Stable coin" && !isActive)
      setIneligibleText({
        title: ERROR_MESSAGES.ETH_WALLET_NOT_CONNECTED.title,
        textBelow: ERROR_MESSAGES.ETH_WALLET_NOT_CONNECTED.message,
      });

    if (!isEthAccountEligible && !isPicassoAccountEligible)
      setIneligibleText({
        title: ERROR_MESSAGES.WRONG_ADDRESS.title,
        textBelow: ERROR_MESSAGES.WRONG_ADDRESS.message,
      });
  }, [
    isEthAccountEligible,
    isPicassoAccountEligible,
    flow,
    isActive,
    extensionStatus,
  ]);

  const ethAssociatedOrSelectedAccount = useEthereumAssociatedAccount(
    account?.toLowerCase(),
    selectedAccount,
    accounts
  );

  const hasNothingToClaim = useCallback(() => {
    if (extensionStatus !== "connected" || !selectedAccount) return true;
    if (!isEthAccountEligible && !isPicassoAccountEligible) return true;

    return false;
  }, [
    extensionStatus,
    selectedAccount,
    isEthAccountEligible,
    isPicassoAccountEligible,
  ]);

  const nextStep = useCrowdloanNextStep(
    selectedAccount?.address,
    account?.toLowerCase()
  );

  const useAssociate = useCrowdloanRewardsAssociate({
    connectedAccounts: accounts,
    api: parachainApi,
    executor,
    selectedPicassoAddress: selectedAccount?.address,
    associateMode:
      nextStep === CrowdloanStep.AssociateEth
        ? "ethereum"
        : nextStep === CrowdloanStep.AssociateKsm
        ? "kusama"
        : undefined,
  });

  const claim = useCrowdloanRewardsClaim({
    api: parachainApi,
    executor,
    selectedPicassoAddress: selectedAccount?.address,
    selectedEthereumAddress: account,
  });

  const availableToClaim = useClaimableAmount(
    nextStep,
    account?.toLowerCase(),
    selectedAccount?.address,
    parachainApi,
    initialPayment
  );

  const claimedRewards = useClaimedAmount(
    account?.toLowerCase(),
    selectedAccount?.address,
    parachainApi
  );

  const { contributedAmount, totalRewards } = useCrowdloanContributions(
    nextStep,
    account?.toLowerCase(),
    selectedAccount?.address
  );

  const hasStarted = useHasStartedCrowdloan(parachainApi);

  const breadcrumbs = [
    <Link key="Overview" underline="none" color="primary" href="/">
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
      {flow}
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
                isClaiming={isPendingAssociate || isPendingClaim}
                SS58Address={
                  ethAssociatedOrSelectedAccount
                    ? `${ethAssociatedOrSelectedAccount.address} (${ethAssociatedOrSelectedAccount.name})`
                    : "-"
                }
                disabled={!hasStarted || availableToClaim.eq(0)}
                claimedRewards={claimedRewards}
                amountContributed={contributedAmount}
                availableToClaim={availableToClaim}
                totalRewards={totalRewards}
                readonlyCrowdLoanContribution={true}
                readonlyAvailableToClaim
                readonlyTotalPicaVested
                picassoAccountName={
                  selectedAccount ? selectedAccount.name : "-"
                }
                readonlySS8Address
                onClaim={async () => {
                  nextStep === CrowdloanStep.AssociateEth
                    ? signEthereum().then(useAssociate)
                    : nextStep === CrowdloanStep.AssociateKsm
                    ? signPolkadotJs().then(useAssociate)
                    : nextStep === CrowdloanStep.Claim
                    ? claim().catch(console.error)
                    : undefined;
                }}
              />
            ) : (
              <KSMClaimForm
                isClaiming={isPendingAssociate || isPendingClaim}
                disabled={!hasStarted || availableToClaim.eq(0)}
                claimedRewards={claimedRewards}
                amountContributed={contributedAmount}
                availableToClaim={availableToClaim}
                totalRewards={totalRewards}
                readonlyCrowdLoanContribution={true}
                readonlyAvailableToClaim
                readonlyTotalPicaVested
                picassoAccountName={
                  selectedAccount ? selectedAccount.name : "-"
                }
                readonlySS8Address
                onClaim={async () => {
                  nextStep === CrowdloanStep.AssociateEth
                    ? signEthereum().then(useAssociate)
                    : nextStep === CrowdloanStep.AssociateKsm
                    ? signPolkadotJs().then(useAssociate)
                    : nextStep === CrowdloanStep.Claim
                    ? claim().catch(console.error)
                    : undefined;
                }}
              />
            )}
          </Grid>
        )}
        <Grid item {...standardPageSize}>
          <SS8WalletHelper />
        </Grid>
      </Grid>
    </DefaultLayout>
  );
};

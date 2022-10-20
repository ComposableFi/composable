import {
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
import { Grid, Typography, useTheme } from "@mui/material";
import { stringToHex } from "@polkadot/util";
import { useCallback, useEffect, useMemo, useState } from "react";
import { crowdLoanSignableMessage } from "@/utils/crowdloanRewards";
import { useRouter } from "next/router";
import { ConnectorType, useBlockchainProvider, useConnector } from "bi-lib";
import {
  useConnectedAccounts,
  useDotSamaContext,
  useExecutor,
  usePendingExtrinsic,
} from "substrate-react";
import { useSnackbar } from "notistack";
import {
  CrowdloanStep,
  useCrowdloanRewardsSlice,
} from "@/stores/defi/polkadot/crowdloanRewards/crowdloanRewards.slice";
import { useCrowdloanRewardsClaim } from "@/defi/polkadot/hooks/crowdloanRewards/useCrowdloanRewardsClaim";
import { useCrowdloanRewardsAssociate } from "@/defi/polkadot/hooks/crowdloanRewards/useCrowdloanRewardsAssociate";
import {
  useCrowdloanRewardsStepGivenConnectedAccounts,
  useCrowdloanRewardsClaimableRewards,
  useCrowdloanRewardsClaimedRewards,
  useCrowdloanRewardsContributionAndRewards,
  useCrowdloanRewardsEligibility,
  useCrowdloanRewardsEthereumAddressAssociatedAccount,
  useCrowdloanRewardsHasStarted,
} from "@/stores/defi/polkadot/crowdloanRewards/hooks";
import { DEFAULT_EVM_ID, APP_NAME, DEFAULT_NETWORK_ID } from "@/defi/polkadot/constants";

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

export const ClaimLoanPage = () => {
  const router = useRouter();
  const { enqueueSnackbar } = useSnackbar();
  const { isActive } = useConnector(ConnectorType.MetaMask);
  const { signer, account } = useBlockchainProvider(DEFAULT_EVM_ID);
  const { extensionStatus } = useDotSamaContext();
  const { parachainApi } = usePicassoProvider();
  const accounts = useConnectedAccounts(DEFAULT_NETWORK_ID);
  const { initialPayment } = useCrowdloanRewardsSlice();
  const executor = useExecutor();
  const selectedAccount = useSelectedAccount();
  const theme = useTheme();
  const hasStarted = useCrowdloanRewardsHasStarted(parachainApi);

  const [ineligibleText, setIneligibleText] = useState({
    title: ERROR_MESSAGES.KSM_WALLET_NOT_CONNECTED.title,
    textBelow: ERROR_MESSAGES.KSM_WALLET_NOT_CONNECTED.message,
  });

  const updateBalance = useStore(
    ({ substrateBalances }) => substrateBalances.updateBalance
  );

  const isPendingClaim = usePendingExtrinsic(
    "claim",
    "crowdloanRewards",
    selectedAccount?.address ?? "-"
  );

  const isPendingAssociate = usePendingExtrinsic(
    "associate",
    "crowdloanRewards",
    ""
  );

  const ethAssociatedOrSelectedAccount =
    useCrowdloanRewardsEthereumAddressAssociatedAccount(
      account?.toLowerCase(),
      selectedAccount,
      accounts
    );

  const nextStep = useCrowdloanRewardsStepGivenConnectedAccounts(
    selectedAccount?.address,
    account?.toLowerCase()
  );

  const availableToClaim = useCrowdloanRewardsClaimableRewards(
    nextStep,
    account?.toLowerCase(),
    selectedAccount?.address,
    parachainApi,
    initialPayment
  );

  const claimedRewards = useCrowdloanRewardsClaimedRewards(
    account?.toLowerCase(),
    selectedAccount?.address,
    parachainApi
  );

  const { isEthAccountEligible, isPicassoAccountEligible } =
    useCrowdloanRewardsEligibility(
      account?.toLowerCase(),
      selectedAccount?.address
    );

  const { contributedAmount, totalRewards } =
    useCrowdloanRewardsContributionAndRewards(
      nextStep,
      account?.toLowerCase(),
      selectedAccount?.address
    );

  const flow = useMemo(() => {
    const pathNames = router.pathname.split("/");
    const pathName = pathNames[pathNames.length - 1];

    if (pathName.toLowerCase() === "ksm") {
      return "KSM";
    } else {
      return "Stable coin";
    }
  }, [router]);

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

  const hasNothingToClaim = useMemo(() => {
    if (extensionStatus !== "connected" || !selectedAccount) return true;
    if (!isEthAccountEligible && !isPicassoAccountEligible) return true;

    return false;
  }, [
    extensionStatus,
    selectedAccount,
    isEthAccountEligible,
    isPicassoAccountEligible,
  ]);

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

  const isStable = isEthAccountEligible;

  const standardPageSize = {
    xs: 12,
  };

  const operation = useCallback(async () => {
    switch (nextStep) {
      case CrowdloanStep.Claim:
        claim().catch(console.error);
        break;
      case CrowdloanStep.AssociateEth:
        signEthereum().then(useAssociate);
        break;
      case CrowdloanStep.AssociateKsm:
        signPolkadotJs().then(useAssociate);
        break;
      default:
        return;
    }
  }, [nextStep, claim, signEthereum, signPolkadotJs, useAssociate]);

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
        {hasNothingToClaim && (
          <Grid item {...standardPageSize} mt={theme.spacing(9)}>
            <NoEligibleWalletFeaturedBox
              title={ineligibleText.title}
              textBelow={ineligibleText.textBelow}
            />
          </Grid>
        )}
        {!hasNothingToClaim && (
          <Grid item {...standardPageSize} mt={theme.spacing(9)}>
            {isStable ? (
              <StablecoinClaimForm
                isClaiming={isPendingAssociate || isPendingClaim}
                SS58Address={
                  ethAssociatedOrSelectedAccount
                    ? `${ethAssociatedOrSelectedAccount.address} (${ethAssociatedOrSelectedAccount.meta.name ?? ""})`
                    : "-"
                }
                disabled={
                  !hasStarted ||
                  availableToClaim.eq(0) ||
                  nextStep === CrowdloanStep.None
                }
                claimedRewards={claimedRewards}
                amountContributed={contributedAmount}
                availableToClaim={availableToClaim}
                totalRewards={totalRewards}
                readonlyCrowdLoanContribution={true}
                readonlyAvailableToClaim
                readonlyTotalPicaVested
                picassoAccountName={
                  selectedAccount && selectedAccount.meta.name  ? selectedAccount.meta.name : "-"
                }
                readonlySS8Address
                onClaim={operation}
              />
            ) : (
              <KSMClaimForm
                isClaiming={isPendingAssociate || isPendingClaim}
                disabled={
                  !hasStarted ||
                  availableToClaim.eq(0) ||
                  nextStep === CrowdloanStep.None
                }
                claimedRewards={claimedRewards}
                amountContributed={contributedAmount}
                availableToClaim={availableToClaim}
                totalRewards={totalRewards}
                readonlyCrowdLoanContribution={true}
                readonlyAvailableToClaim
                readonlyTotalPicaVested
                picassoAccountName={
                  selectedAccount && selectedAccount.meta.name ? selectedAccount.meta.name : "-"
                }
                readonlySS8Address
                onClaim={operation}
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

export default ClaimLoanPage;

import {
  AlertBox,
  DefaultLayout,
  KSMClaimForm,
  Link,
  NoEligibleWalletFeaturedBox,
  PageTitle,
  SS8WalletHelper,
  StablecoinClaimForm
} from "@/components";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { useStore } from "@/stores/root";
import { alpha, Grid, Typography, useTheme } from "@mui/material";
import { stringToHex } from "@polkadot/util";
import { useCallback, useMemo, useState } from "react";
import { crowdLoanSignableMessage } from "@/utils/crowdloanRewards";
import { useRouter } from "next/router";
import { ConnectorType, useBlockchainProvider, useConnector } from "bi-lib";
import { OpenInNewRounded } from "@mui/icons-material";
import { useDotSamaContext, useExecutor, usePendingExtrinsic } from "substrate-react";
import { useSnackbar } from "notistack";
import BigNumber from "bignumber.js";
import CheckCircleOutlineIcon from "@mui/icons-material/CheckCircleOutline";
import {
  useAccountState,
  useCrowdloanRewardsSlice,
} from "@/stores/defi/polkadot/crowdloanRewards/crowdloanRewards.slice";
import { useCrowdloanRewardsAssociate } from "@/defi/polkadot/hooks/useCrowdloanRewardsAssociate";

const DEFAULT_EVM_ID = 1;
const APP_NAME = "Picasso UI";

const ERROR_MESSAGES = {
  KSM_WALLET_NOT_CONNECTED: {
    message:
      "Please connect the KSM address used to contribute to the Picasso crowdloan.",
    title: "Nothing to claim"
  },
  WRONG_ADDRESS: {
    message:
      "Please connect the address used to contribute to Picasso crowdloan.",
    title: "Nothing to claim"
  },
  ETH_WALLET_NOT_CONNECTED: {
    message: "Please connect metamask to claim PICA rewards.",
    title: "Nothing to claim"
  }
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
    textBelow: ERROR_MESSAGES.KSM_WALLET_NOT_CONNECTED.message
  });
  const [showAlertBox, setShowAlertBox] = useState<boolean>(false);

  const hasNothingToClaim = (): boolean => {
    return extensionStatus !== "connected";
  };

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
      const { web3FromAddress, web3Enable } = require("@polkadot/extension-dapp");
      await web3Enable(APP_NAME);
      if (!selectedAccount || !parachainApi) throw new Error('Missing Connection')
      const injector = await web3FromAddress(selectedAccount.address);
      if (!injector.signer.signRaw) throw new Error('Missing Connection')
      if (!parachainApi || !selectedAccount) throw new Error('Missing Connection');
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
      if (!parachainApi || !signer || !selectedAccount) throw new Error('Missing Connection');
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
  const ethereumAccountState = useAccountState(account ?? "-", "ethereum");
  const ksmAccountState = useAccountState(
    selectedAccount?.address ?? "-",
    "kusama"
  );

  const [methodToCall, setMethodToCall] = useState<
    "associateEth" | "associateKSM" | "claim" | "none"
  >("none");

  const useAssociate = useCrowdloanRewardsAssociate({
    api: parachainApi,
    executor,
    selectedPicassoAddress: selectedAccount?.address,
    selectedEthereumAddress: account,
    associateMode:
      methodToCall === "associateEth"
        ? "ethereum"
        : methodToCall === "associateKSM"
        ? "kusama"
        : undefined,
  });

  const {
    claimedRewards,
    amountContributed,
    availableToClaim,
    totalRewards,
  } = useMemo(() => {
    let claimedRewards = new BigNumber(0);
    let amountContributed = new BigNumber(0);
    let availableToClaim = new BigNumber(0);
    let totalRewards = new BigNumber(0);

    if (ksmAccountState) {
      if (ksmAccountState.crowdloanSelectedAccountStatus === "ineligible") {
        if (
          ethereumAccountState &&
          ethereumAccountState.crowdloanSelectedAccountStatus === "canAssociate"
        ) {
          claimedRewards = ethereumAccountState.claimedRewards;
          amountContributed = ethereumAccountState.amountContributed;
          availableToClaim = ethereumAccountState.totalRewards.times(
            initialPayment
          );
          totalRewards = ethereumAccountState.totalRewards;
          setMethodToCall("associateEth");
          // allow associate eth
        }
        // check if ethereum is eligible and can be associated
        // if ethereum is not eligible then we leave everything 0
      } else if (
        !ethereumAccountState ||
        (ethereumAccountState &&
          ethereumAccountState.crowdloanSelectedAccountStatus === "ineligible")
      ) {
        // in this case ethereum account is not eligible and is connected as well
        if (ksmAccountState.crowdloanSelectedAccountStatus === "canAssociate") {
          claimedRewards = ksmAccountState.claimedRewards;
          amountContributed = ksmAccountState.amountContributed;
          availableToClaim = ksmAccountState.totalRewards.times(initialPayment);
          totalRewards = ksmAccountState.totalRewards;
          setMethodToCall("associateKSM");
        } else {
          claimedRewards = ksmAccountState.claimedRewards;
          amountContributed = ksmAccountState.amountContributed;
          availableToClaim = ksmAccountState.availableToClaim;
          totalRewards = ksmAccountState.totalRewards;
          setMethodToCall("claim");
        }
      }
    } else {
      setMethodToCall("none");
    }

    return {
      claimedRewards,
      amountContributed,
      availableToClaim,
      totalRewards,
    };
  }, [ethereumAccountState, ksmAccountState, initialPayment]);

  const breadcrumbs = [
    <Link
      key="Overview"
      underline="none"
      color="primary"
      href="/"
    >
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
    xs: 12
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
                SS58Address={selectedAccount ? selectedAccount.address : "-"}
                disabled={false}
                claimedRewards={claimedRewards}
                amountContributed={amountContributed}
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
                  methodToCall === "associateEth"
                    ? signEthereum().then(useAssociate)
                    : methodToCall === "associateKSM"
                    ? signPolkadotJs().then(useAssociate)
                    : undefined;
                }}
                onChange={(name: string, value: unknown) => {
                  console.log("Change", name, value);
                }}
              />
            ) : (
              <KSMClaimForm
                disabled={false}
                claimedRewards={claimedRewards}
                amountContributed={amountContributed}
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
                  methodToCall === "associateEth"
                    ? signEthereum().then(useAssociate)
                    : methodToCall === "associateKSM"
                    ? signPolkadotJs().then(useAssociate)
                    : undefined;
                }}
                onChange={(name: string, value: unknown) => {
                  console.log("Change", name, value);
                }}
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
                    )
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
              Transaction successful
            </AlertBox>
          </Grid>
        )}
      </Grid>
    </DefaultLayout>
  );
};

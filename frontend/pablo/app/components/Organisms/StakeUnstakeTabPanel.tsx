import { Box, Button, Typography, useTheme } from "@mui/material";
import { useMobile } from "@/hooks/responsive";
import { DropdownCombinedBigNumberInput } from "../Molecules";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { useEffect, useState } from "react";
import BigNumber from "bignumber.js";
import { TokenId } from "@/defi/types";
import { getToken, TOKEN_IDS } from "@/defi/Tokens";
import { useAppSelector } from "@/hooks/store";

export type StakeUnstakeTabPanelProps = { activeTab: "staking" | "unstaking" };

export const StakeUnstakeTabPanel: React.FC<StakeUnstakeTabPanelProps> = ({
  activeTab,
}) => {
  const theme = useTheme();
  const isMobile = useMobile();
  const [balance1] = useState<BigNumber>(new BigNumber(200.0));
  const [amount1, setAmount1] = useState<BigNumber>(new BigNumber(0));
  const [valid1, setValid1] = useState<boolean>(false);
  const [tokenId1, setTokenId1] = useState<TokenId>("pablo");
  const [isApproved, setIsApproved] = useState<boolean>(false);
  const userStakeInfo = useAppSelector((state) => state.polkadot.userStakeInfo);
  const [buttonText, setButtonText] = useState("Approve");

  const handleClick = () => {
    if (buttonText === "Stake") {
      // TODO : Stake Call
    } else if (buttonText === "Approve") {
      setIsApproved(true);
    } else {
      // TODO : Unstake Call
    }
  };

  useEffect(() => {
    if (activeTab === "staking") {
      setButtonText(isApproved ? "Stake" : "Approve");
    } else {
      setButtonText("Unstake");
    }
  }, [activeTab, isApproved]);

  return (
    <Box>
      <DropdownCombinedBigNumberInput
        maxValue={balance1}
        setValid={setValid1}
        noBorder
        value={amount1}
        setValue={setAmount1}
        buttonLabel={"Max"}
        ButtonProps={{
          onClick: () => setAmount1(balance1),
          sx: {
            padding: theme.spacing(1),
          },
        }}
        CombinedSelectProps={{
          value: tokenId1,
          setValue: setTokenId1,
          dropdownModal: true,
          forceHiddenLabel: isMobile ? true : false,
          options: [
            {
              value: "none",
              label: "Select",
              icon: undefined,
              disabled: true,
              hidden: true,
            },
            ...TOKEN_IDS.map((tokenId) => ({
              value: tokenId,
              label: getToken(tokenId).symbol,
              icon: getToken(tokenId).icon,
            })),
          ],
          borderLeft: false,
          minWidth: isMobile ? undefined : 150,
          searchable: true,
        }}
        LabelProps={{
          label: "Amount",
          BalanceProps: {
            title: <AccountBalanceWalletIcon color="primary" />,
            balance: `${balance1}`,
          },
        }}
      />
      <Box mt={3}>
        <Button onClick={handleClick} fullWidth variant="contained">
          {buttonText}
        </Button>
      </Box>
      <Box display="flex" mt={6} justifyContent="space-between">
        <Box>
          <Typography variant="body2">Your balance</Typography>
        </Box>
        <Box>
          <Typography variant="body2">
            {userStakeInfo.balance.toFormat()} PABLO
          </Typography>
        </Box>
      </Box>
      <Box display="flex" mt={2} justifyContent="space-between">
        <Box>
          <Typography variant="body2">Your staked balance</Typography>
        </Box>
        <Box>
          <Typography variant="body2">
            {userStakeInfo.stakedBalance.toFormat()} sPABLO
          </Typography>
        </Box>
      </Box>
      <Box display="flex" mt={2} justifyContent="space-between">
        <Box>
          <Typography variant="body2">Next reward amount</Typography>
        </Box>
        <Box>
          <Typography variant="body2">
            {userStakeInfo.nextRewardAmount.toFormat()} sPABLO
          </Typography>
        </Box>
      </Box>
      <Box display="flex" mt={2} justifyContent="space-between">
        <Box>
          <Typography variant="body2">5-day ROI</Typography>
        </Box>
        <Box>
          <Typography variant="body2">{userStakeInfo.roi}%</Typography>
        </Box>
      </Box>
    </Box>
  );
};

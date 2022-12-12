import { Badge } from "../Atoms/Badge";
import { trimAddress, WalletViewTabs } from "../WalletViewModal";
import { TabPanel } from "../Atoms/TabPanel";
import Image from "next/image";

import { ContentCopy, OpenInNew } from "@mui/icons-material";
import { alpha, Box, IconButton, Typography, useTheme } from "@mui/material";
import { EthereumWallet } from "../../types";

export type EthereumAccountViewProps = {
  activePanel: WalletViewTabs;
  selectedEthereumWallet: EthereumWallet;
  connectedEthereumAccount: string;
  onDisconnectWallet: (() => void) | undefined;
  etherscanUrl: string;
};

export const EthereumAccountView = ({
  activePanel,
  connectedEthereumAccount,
  selectedEthereumWallet,
  etherscanUrl,
  onDisconnectWallet,
}: EthereumAccountViewProps) => {
  const theme = useTheme();

  return (
    <TabPanel value={activePanel} index={WalletViewTabs.Wallets}>
      <Box>
        <Typography variant="body2">Connected with</Typography>
        <Badge
          marginLeft={theme.spacing(1)}
          label={selectedEthereumWallet.name}
          icon={
            <Image
              src={selectedEthereumWallet.icon}
              height="16px"
              width="16px"
            />
          }
          color={theme.palette.text.primary}
          background={alpha(theme.palette.text.primary, 0.1)}
        />
      </Box>
      <Box marginTop={theme.spacing(2)} display="flex" alignItems={"center"}>
        <Image height="32" width="32" src="networks/mainnet.svg"></Image>
        <Typography marginLeft={theme.spacing(1)} variant="body2">
          {trimAddress(connectedEthereumAccount)}
        </Typography>
        <IconButton
          onClick={async (_evt) => {
            await navigator.clipboard.writeText(connectedEthereumAccount);
          }}
          color="primary"
          size="small"
        >
          <ContentCopy></ContentCopy>
        </IconButton>
        <IconButton
          onClick={(_evt) => {
            window.open(etherscanUrl + "address/" + connectedEthereumAccount);
          }}
          color="primary"
          size="small"
        >
          <OpenInNew></OpenInNew>
        </IconButton>
      </Box>
      <Box marginTop={theme.spacing(2)}>
        <Typography
          onClick={onDisconnectWallet}
          color={theme.palette.text.secondary}
          variant="body2"
        >
          Disconnect
        </Typography>
      </Box>
    </TabPanel>
  );
};

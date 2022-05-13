import React from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import { Label, BaseAsset } from "@/components/Atoms";
import { getToken } from "@/defi/Tokens";
import { TokenId } from "@/defi/types";
import { 
  alpha,
  Box,
  IconButton,
  Typography,
  useTheme,
  Button, 
} from "@mui/material";
import CloseIcon from "@mui/icons-material/Close";

import { useDispatch } from "react-redux";
import {  
  closePreviewSupplyModal,
  openConfirmingSupplyModal, 
} from "@/stores/ui/uiSlice";
import {  
  setCurrentSupply, 
} from "@/stores/defi/pool";
import { YourPosition } from "../YourPosition";
import { useAppSelector } from "@/hooks/store";

export const PreviewSupplyModal: React.FC<ModalProps> = ({
  ...rest
}) => {
  const theme = useTheme();
  const dispatch = useDispatch();

  const {
    tokenId1,
    tokenId2,
    pooledAmount1,
    pooledAmount2,
    price1,
    price2,
    amount,
    share,
  } = useAppSelector((state) => state.pool.currentSupply);

  const token1 = getToken(tokenId1 as TokenId);
  const token2 = getToken(tokenId2 as TokenId);

  const confirmSupply = () => {
    dispatch(closePreviewSupplyModal());
    dispatch(openConfirmingSupplyModal());

    setTimeout(() => {
      dispatch(setCurrentSupply({confirmed: true}))
    }, 2000)
  };

  return (
    <Modal
      onClose={() => dispatch(closePreviewSupplyModal())}
      {...rest}
      PaperProps={{
        sx: {
          "& > .MuiBox-root": {
            height: 'auto',
          },
        }
      }}
    >
      <Box
        sx={{
          background: theme.palette.gradient.secondary,
          boxShadow: `-1px -1px ${alpha(theme.palette.common.white, theme.custom.opacity.light)}`,
          width: 550,
          [theme.breakpoints.down('sm')]: {
            width: '100%',
          },
          borderRadius: 1,
          padding: theme.spacing(3),
          marginBottom: theme.spacing(4),
          marginTop: theme.spacing(4),
        }}
      >
        <Box
          display="flex"
          alignItems="center"
          justifyContent="space-between"
        >
          <Typography variant="body1">
            You will recieve
          </Typography>
          <IconButton 
            onClick={() => dispatch(closePreviewSupplyModal())}
          >
            <CloseIcon />
          </IconButton>
        </Box>

        <Typography variant="h5" mt={1.75}>
          {`${amount}`}
        </Typography>

        <Typography variant="body1" color="text.secondary" mt={1.75}>
          {`LP ${token2?.symbol}/${token1?.symbol} Tokens`}
        </Typography>

        <Typography variant="body2" mt={4} textAlign="center" paddingX={4.25}>
          Output is estimated. If the price changes by more than 5% your transaction will revert.
        </Typography>

        <Box
          mt={4}
          borderTop={`1px solid ${alpha(theme.palette.common.white, theme.custom.opacity.main)}`}
        />

        <Label
          mt={4}
          label={`Pooled ${token2?.symbol}`}
          BalanceProps={{
            title: <BaseAsset icon={token2?.icon} pr={1} />,
            balance: `${pooledAmount1}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label={`Pooled ${token1?.symbol}`}
          BalanceProps={{
            title: <BaseAsset icon={token1?.icon} pr={1} />,
            balance: `${pooledAmount2}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label={`Price`}
          BalanceProps={{
            balance: `1 ${token2?.symbol} = ${price2} ${token1?.symbol}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label=""
          BalanceProps={{
            balance: `1 ${token1?.symbol} = ${price1} ${token2?.symbol}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label={`Share of pool`}
          BalanceProps={{
            balance: `${share}%`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Box 
          display="flex"
          justifyContent="space-between"
          alignItems="center"
          mt={4}
          gap={2}
        >
          <Box width="50%">
            <Button 
              variant="contained" 
              size="large"
              fullWidth
              onClick={confirmSupply}
            >
              Auto bond
            </Button>
          </Box>

          <Box width="50%">
            <Button 
              variant="text" 
              size="large"
              fullWidth
              onClick={() => dispatch(closePreviewSupplyModal())}
            >
              No, thanks
            </Button>
          </Box>
        </Box>

        <YourPosition
          noTitle={false}
          tokenId1={tokenId1 as TokenId}
          tokenId2={tokenId2 as TokenId}
          pooledAmount1={pooledAmount1}
          pooledAmount2={pooledAmount2}
          amount={amount}
          share={share}
          mt={4}
        />

      </Box>
    </Modal>  
  );
};


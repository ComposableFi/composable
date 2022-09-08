import React from "react";
import { CircularProgress } from "@/components/Atoms";
import { ModalProps, Modal, Link } from "@/components/Molecules";
import { 
  alpha,
  Box,
  Typography,
  useTheme,
  Button, 
} from "@mui/material";
import CheckCircleOutlineIcon from '@mui/icons-material/CheckCircleOutline';
import { useAppSelector } from "@/hooks/store";

import { useDispatch } from "react-redux";
import {  
  closeConfirmingSupplyModal,
} from "@/stores/ui/uiSlice";
import { SupplyModalProps } from "./ConfirmSupplyModal";

export const ConfirmingSupplyModal: React.FC<SupplyModalProps & ModalProps> = ({
  assetOne,
  assetTwo,
  assetOneAmount,
  assetTwoAmount,
  lpReceiveAmount,
  priceOneInTwo,
  priceTwoInOne,
  share,
  ...rest
}) => {
  const theme = useTheme();
  const dispatch = useDispatch();

  const {
    confirmed,
  } = useAppSelector((state) => state.pool.currentSupply);


  const handelClose = () => {
    dispatch(closeConfirmingSupplyModal());
  }

  return (
    <Modal
      onClose={() => handelClose()}
      {...rest}
    >
      {!confirmed && (
        <Box
          textAlign="center"
          sx={{
            width: 550,
            [theme.breakpoints.down('sm')]: {
              width: '100%',
            },
            padding: theme.spacing(3)
          }}
        >
          <Box display="flex" justifyContent="center">
            <CircularProgress size={96} />
          </Box>
          <Typography variant="h5" mt={8}>
            Waiting for confirmation
          </Typography>
          <Typography variant="subtitle1" mt={2} color="text.secondary">
            Adding {`${assetOneAmount}`} {assetOne?.symbol} and {`${assetTwoAmount}`} {assetTwo?.symbol}
          </Typography>
          <Typography 
            variant="body1" 
            mt={2}
            sx={{
              color: alpha(theme.palette.common.white, theme.custom.opacity.main),
            }}
          >
            Confirming this transaction in your wallet
          </Typography>
        </Box>
      )}

      {confirmed && (
        <Box
          textAlign="center"
          sx={{
            width: 550,
            [theme.breakpoints.down('sm')]: {
              width: '100%',
            },
            padding: theme.spacing(3)
          }}
        >
          <Box>
            <CheckCircleOutlineIcon
              sx={{
                fontSize: 96,
                color: theme.palette.primary.main,
              }}
            />
          </Box>
          <Typography variant="h5" mt={8}>
            Transaction Submitted
          </Typography>
          <Box display="flex" justifyContent="center" mt={2} mb={8}>
            <Link target="_blank" href="/frontend/apps/pablo/pages">
              View on Polkadot {"{.js}"}
            </Link>
          </Box>
          
          <Button 
            variant="outlined" 
            size="large" 
            fullWidth
            onClick={() => handelClose()}
          >
            Close
          </Button>      
        </Box>
      )}
    </Modal>  
  );
};

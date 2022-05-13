import React from "react";
import { CircularProgress } from "@/components/Atoms";
import { ModalProps, Modal, Link } from "@/components/Molecules";
import { getToken } from "@/defi/Tokens";
import { TokenId } from "@/defi/types";
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

export const ConfirmingSupplyModal: React.FC<ModalProps> = ({
  ...rest
}) => {
  const theme = useTheme();
  const dispatch = useDispatch();

  const {
    tokenId1,
    tokenId2,
    pooledAmount1,
    pooledAmount2,
    confirmed,
  } = useAppSelector((state) => state.pool.currentSupply);

  const token1 = getToken(tokenId1 as TokenId);
  const token2 = getToken(tokenId2 as TokenId);

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
            Removing {`${pooledAmount2}`} {token2?.symbol} and {`${pooledAmount1}`} {token1?.symbol}
          </Typography>
          <Typography 
            variant="body1" 
            mt={2}
            sx={{
              color: alpha(theme.palette.common.white, theme.custom.opacity.main),
            }}
          >
            Confirm this transaction in your wallet
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
            <Link target="_blank" href="/">
              View on Polacdot {"{.js}"}
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


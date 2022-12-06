import React from "react";
import { CircularProgress } from "@/components/Atoms";
import { ModalProps, Modal } from "@/components/Molecules";
import { 
  alpha,
  Box,
  Typography,
  useTheme,
} from "@mui/material";
import { SupplyModalProps } from "./ConfirmSupplyModal";
import { setUiState } from "@/store/ui/ui.slice";

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

  const handelClose = () => {
    setUiState({ isConfirmingSupplyModalOpen: false });
  }

  return (
    <Modal
      onClose={() => handelClose()}
      {...rest}
    >
      {/* {!confirmed && ( */}
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
            Adding {`${assetOneAmount}`} {assetOne?.getSymbol()} and {`${assetTwoAmount}`} {assetTwo?.getSymbol()}
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
      {/* )} */}

      {/* {confirmed && (
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
      )} */}
    </Modal>  
  );
};

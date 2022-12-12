import type { NextPage } from "next";
import { Container, Box, Grid } from "@mui/material";
import Default from "@/components/Templates/Default";
import AccountSettings from "@/components/Organisms/TransactionSettings/AccountSettings";
import { PageTitle } from "@/components";
import SwapForm from "@/components/Organisms/swap/SwapForm";
import SwapChart from "@/components/Organisms/swap/SwapChart";

const twoColumnPageSize = {
  xs: 12,
  md: 12,
  lg: 6,
};

const Swap: NextPage = () => {
  return (
    <Default>
      <Container maxWidth="lg">
        <Box mb={25}>
          <Box textAlign="center">
            <PageTitle
              title="Swap"
            />
          </Box>
          <Grid mt={4} container spacing={4}>
            <Grid item {...twoColumnPageSize}>
              <SwapChart height={610}/>
            </Grid>
            <Grid item {...twoColumnPageSize}>
              <SwapForm />
            </Grid>
          </Grid>
          {/* {message.text && (
            <Box sx={{maxWidth: 854, margin: "auto"}}>
              <Alert
                severity={message.severity}
                alertText={message.text}
                onClose={() => dispatch(setMessage({}))}
                underlined
                centered
                action={
                  message.link ? (
                    <Link
                      href={message.link}
                      target="_blank"
                      rel="noopener"
                    >
                      <OpenInNewRoundedIcon />
                    </Link>
                  ) : undefined
                }
              />
            </Box>
          )} */}
          <AccountSettings />
        </Box>
      </Container>
    </Default>
  );
};

export default Swap;

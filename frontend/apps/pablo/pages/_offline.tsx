import * as React from "react";
import type { NextPage } from "next";
import { Container, Typography, Box } from "@mui/material";

const Offline: NextPage = () => {
  return (
    <Container maxWidth="lg">
      <Box
        sx={{
          my: 4,
          display: "flex",
          flexDirection: "column",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <Typography variant="h4" component="h1" gutterBottom>
          It seems like you are offline.
        </Typography>
      </Box>
    </Container>
  );
};

export default Offline;

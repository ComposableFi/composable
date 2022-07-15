import type { NextPage } from "next";
import { Container, Typography, Box } from "@mui/material";
import Default from "@/components/Templates/Default";
import { Link } from "@/components";
import { ImportPool } from "@/components/Organisms/ImportPool";

const ImportPoolPage: NextPage = () => {
  const breadcrumbs = [
    <Link key="pool" underline="none" color="primary" href="/pool">
      Pool
    </Link>,
    <Typography key="import" color="text.primary">
      Import pool
    </Typography>,
  ];
  return (
    <Default breadcrumbs={breadcrumbs}>
      <Container maxWidth="lg">
        <Box
          sx={{
            display: "flex",
            flexDirection: "column",
            justifyContent: "center",
            alignItems: "center",
            marginBottom: 18,
          }}
        >
          <ImportPool />
        </Box>
      </Container>
    </Default>
  );
};

export default ImportPoolPage;

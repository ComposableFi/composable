import { Container, Skeleton } from "@mui/material";

export const RemoveLiquiditySkeleton = () => {
  return (
    <Container maxWidth={"sm"}>
      <Skeleton variant="rectangular" width={"100%"} height={"100%"} />
    </Container>
  );
};

import { Box, Grid, Skeleton } from "@mui/material";
import { FC } from "react";
import { useTheme } from "@mui/material/styles";

export const BondDetailSkeleton: FC<{}> = () => {
  const theme = useTheme();
  return (
    <Box
      display={"flex"}
      width="100%"
      alignItems="center"
      justifyContent="center"
    >
      <Grid
        container
        maxWidth={1032}
        display="flex"
        justifyContent="center"
        gap={9}
        mt={9}
      >
        <Grid item width="100%" display="flex" justifyContent="center">
          <Skeleton variant="text" width={270} height={111} />
        </Grid>
        <Grid item display="flex" justifyContent="space-between" width="100%">
          <Skeleton
            variant="rectangular"
            width={234}
            height={118}
            sx={{ borderRadius: `${theme.shape.borderRadius}px` }}
          />
          <Skeleton
            variant="rectangular"
            width={234}
            height={118}
            sx={{ borderRadius: `${theme.shape.borderRadius}px` }}
          />
          <Skeleton
            variant="rectangular"
            width={234}
            height={118}
            sx={{ borderRadius: `${theme.shape.borderRadius}px` }}
          />
          <Skeleton
            variant="rectangular"
            width={234}
            height={118}
            sx={{ borderRadius: `${theme.shape.borderRadius}px` }}
          />
        </Grid>
        <Grid item width="100%" display="flex" justifyContent="center">
          <Skeleton variant="text" width={279} height={118} />
        </Grid>
        <Grid item width="100%" display="flex" justifyContent="center">
          <Skeleton variant="text" width={1032} height={118} />
        </Grid>
        <Grid item width="100%" display="flex" justifyContent="center">
          <Skeleton variant="text" width={1032} height={118} />
        </Grid>
      </Grid>
    </Box>
  );
};

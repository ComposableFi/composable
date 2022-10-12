import { Box, Skeleton, Stack, useTheme } from "@mui/material";

export const ChartLoadingSkeleton = () => {
  const theme = useTheme();
  return (
    <Box
      borderRadius={1}
      padding={6}
      sx={{
        background: theme.palette.background.paper,
      }}
    >
      <Stack direction="row" gap={3}>
        <Skeleton variant="text" height={48} width="50%" />
        <Skeleton variant="text" width={48} />
        <Skeleton variant="text" width={48} />
        <Skeleton variant="text" width={48} />
        <Skeleton variant="text" width={48} />
      </Stack>
      <Box
        sx={{
          height: 330,
          mt: 4,
        }}
      >
        <Skeleton variant="rounded" height={330} width={"100%"} />
      </Box>
    </Box>
  );
};

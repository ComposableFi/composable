import { Stack, Typography } from "@mui/material";

export const TokenWithUSD = ({
  amount,
  symbol,
  price,
}: {
  symbol: string;
  amount: string;
  price: string;
}) => {
  return (
    <Stack direction="row" alignItems="center" gap={1}>
      <Typography variant="body2" color="text.primary">
        {amount} {symbol}
      </Typography>
      <Typography variant="body2" color="text.secondary">
        (~${price})
      </Typography>
    </Stack>
  );
};

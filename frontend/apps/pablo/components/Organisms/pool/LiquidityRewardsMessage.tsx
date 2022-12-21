import { MessageBox } from "@/components";
import { Box } from "@mui/material";
import { useState } from "react";

export const LiquidityRewardsMessage = () => {
  const [messageBoxOpen, setMessageBoxOpen] = useState(true);

  return (
    <Box display="flex" flexDirection="column" alignItems="center" mb={8}>
      {messageBoxOpen && (
        <MessageBox
          title="Liquidity provider rewards"
          message="Liquidity providers earn a 0.3% fee (default for all pairs, subject
            to change) on all trades proportional to their share of the pool.
            Fees are added to the pool, accrue in real time and can be claimed
            by withdrawing your liquidity."
          onClose={() => setMessageBoxOpen(false)}
        />
      )}
    </Box>
  );
};

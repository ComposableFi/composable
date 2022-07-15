import { useTheme, Grid, Typography } from "@mui/material";
import { Link } from "../../Molecules";

export const SS8WalletHelper: React.FC<{}> = () => {
  const theme = useTheme();
  return (
    <Typography variant="body2" textAlign="center">
      To claim your rewards you will need a SS8 wallet address{" "}
      <Link target={"_blank"} href="http://docs.composable.finance/">
        learn more
      </Link>
    </Typography>
  );
};

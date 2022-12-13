import {
  alpha,
  Box,
  BoxProps,
  Theme,
  Typography,
  useTheme,
} from "@mui/material";
import { BaseAsset, Checkbox } from "@/components/Atoms";
import { StakedFinancialNftPosition } from "@/defi/types";
import { useAsset } from "@/defi/hooks";
import { PBLO_ASSET_ID } from "@/defi/utils";

const defaultFlexBoxProps = {
  display: "flex",
  alignItems: "center",
  justifyContent: "space-between",
  gap: 1,
};

const containerProps = (theme: Theme, selected?: boolean) =>
  ({
    py: 1.75,
    pl: 2,
    pr: 3,
    borderRadius: 9999,
    height: 56,
    sx: {
      background: selected
        ? alpha(theme.palette.primary.main, theme.custom.opacity.light)
        : undefined,
      border: `1px solid ${theme.palette.primary.main}`,
    },
    ...defaultFlexBoxProps,
  } as const);

export type CheckableXPabloItemBoxProps = {
  xPablo: StakedFinancialNftPosition;
  selectedXPabloId?: string;
  setSelectedXPabloId?: (id?: string) => void;
} & BoxProps;

export const CheckableXPabloItemBox: React.FC<CheckableXPabloItemBoxProps> = ({
  xPablo,
  selectedXPabloId,
  setSelectedXPabloId,
  ...boxProps
}) => {
  const theme = useTheme();
  const selected = xPablo.nftId === selectedXPabloId;
  const pabloAsset = useAsset(PBLO_ASSET_ID);
  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSelectedXPabloId?.(event.target.checked ? xPablo.nftId : undefined);
  };

  return (
    <Box {...containerProps(theme, selected)} {...boxProps}>
      <Box {...defaultFlexBoxProps}>
        <Checkbox
          value={xPablo.nftId}
          checked={selected}
          onChange={handleChange}
          inputProps={{ "aria-label": "controlled" }}
        />
        <BaseAsset icon={pabloAsset?.getIconUrl()} label={`x-${pabloAsset?.getSymbol()} ${xPablo.nftId}`} />
      </Box>
      <Typography variant="body1">
        {`${xPablo.lockedPrincipalAsset.toFormat()}(~$${xPablo.lockedPrincipalAsset.toFormat()})`}
      </Typography>
    </Box>
  );
};

import { DurationOption } from "@/stores/defi/staking";

export const TAB_ITEMS = [
  {
    label: "Stake and mint",
  },
  {
    label: "Burn and unstake",
  },
];

export const DURATION_OPTION_ITEMS = [
  {
    label: "2 weeks (0x)",
    value: "2w",
  },
  {
    label: "2 months (0.25x)",
    value: "2m",
  },
  {
    label: "1 year (0.5x)",
    value: "1y",
  },
  {
    label: "2 years (1x)",
    value: "2y",
  },
] as { label: string; value: DurationOption }[];

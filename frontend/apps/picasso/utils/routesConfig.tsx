import dynamic from "next/dynamic";
import config from "@/constants/config";
import { IconButton } from "@mui/material";
import { OpenInNew } from "@mui/icons-material";
import Link from "next/link";

type MenuItem = {
  label: string;
  path: string;
  icon: React.ComponentType<any>;
  endAdornment?: React.ReactNode;
  status: "active" | "inactive";
  matches?: string[];
};

type ConfigType = MenuItem & {
  subItems?: Array<MenuItem>;
};

export const routesConfig: ConfigType[] = [
  {
    label: "Overview",
    path: "/",
    icon: dynamic(() => import("@mui/icons-material/HomeRounded")),
    status: "active",
    matches: [
      "/",
      "/crowdloan-rewards",
      "/crowdloan-rewards/ksm",
      "/crowdloan-rewards/stablecoin",
    ],
  },
  {
    label: "Transfers",
    path: "/transfers",
    icon: dynamic(() => import("@mui/icons-material/SwapHorizRounded")),
    status: "active",
    matches: ["/transfers"],
  },
  {
    label: "Stats",
    path: "/stats",
    icon: dynamic(() => import("@mui/icons-material/EqualizerRounded")),
    status: "active",
    matches: ["/stats"],
  },
  {
    label: "Staking",
    path: "/staking",
    icon: dynamic(() => import("@mui/icons-material/TollRounded")),
    status: "active",
    matches: ["/staking"],
  },
  {
    label: "Governance",
    path: config.governanceUrl,
    icon: dynamic(() => import("@mui/icons-material/HowToVoteRounded")),
    status: "active",
    matches: [],
    endAdornment: (
      <a target="_blank" href={config.governanceUrl} rel="noopener noreferrer">
        <IconButton color="primary">
          <OpenInNew />
        </IconButton>
      </a>
    ),
  },
  {
    label: "Pablo",
    path: config.pabloUrl,
    icon: dynamic(() => import("@mui/icons-material/Autorenew")),
    status: "active",
    endAdornment: (
      <Link href="#" passHref>
        <IconButton>
          <OpenInNew />
        </IconButton>
      </Link>
    ),
  },
];
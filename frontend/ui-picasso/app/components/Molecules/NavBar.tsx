import { OpenInNew } from "@mui/icons-material";
import ExpandMore from "@mui/icons-material/ExpandMore";
import {
  Accordion,
  AccordionDetails,
  AccordionSummary,
  Box,
  IconButton,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  Theme,
  useTheme,
} from "@mui/material";
import dynamic from "next/dynamic";
import Link from "next/link";
import { NextRouter, useRouter } from "next/router";
import { Logo } from "../Atom";
import { FC } from "react";

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

const RoutesConfig: ConfigType[] = [
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
    label: "Governance",
    path: "/governance",
    icon: dynamic(() => import("@mui/icons-material/HowToVoteRounded")),
    status: "active",
    matches: ["/governance"],
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
    label: "Bonding",
    path: "/bonding",
    icon: dynamic(() => import("@mui/icons-material/PaymentsRounded")),
    status: "active",
    matches: ["/bonding"],
  },
  {
    label: "Pablo Exchange",
    path: "/pablo-exchange",
    icon: dynamic(() => import("@mui/icons-material/Autorenew")),
    status: "inactive",
    endAdornment: (
      <Link href="#" passHref>
        <IconButton>
          <OpenInNew />
        </IconButton>
      </Link>
    ),
  },
  {
    label: "Mosaic Bridge",
    path: "/mosaic-bridge",
    icon: dynamic(() => import("@mui/icons-material/JoinInner")),
    status: "inactive",
    endAdornment: (
      <Link href="#">
        <IconButton>
          <OpenInNew />
        </IconButton>
      </Link>
    ),
  },
];
type NavItemProps = {
  router: NextRouter;
  config: MenuItem;
  theme: Theme;
  isSubItem: boolean;
};
const NavItem: FC<NavItemProps> = ({ router, config, theme, isSubItem }) => {
  const handleClick = () => {
    if (config.status === "active") {
      router.push(config.path);
    }
  };

  return (
    <ListItem
      selected={config?.matches?.includes(router.pathname)}
      button
      disabled={config.status === "inactive"}
      sx={{ paddingLeft: isSubItem ? "3rem" : "1.5rem" }}
      onClick={handleClick}
    >
      <ListItemIcon>
        <config.icon />
      </ListItemIcon>
      <ListItemText primary={config.label} />
      {config.endAdornment && (
        <ListItemIcon sx={{ "> a": { color: theme.palette.primary.light } }}>
          {config.endAdornment}
        </ListItemIcon>
      )}
    </ListItem>
  );
};

export const NavBar: React.FC = () => {
  const theme = useTheme();
  const router = useRouter();
  return (
    <div>
      <Box
        sx={{
          padding: theme.spacing(6, 3),
          mb: theme.spacing(4),
        }}
      >
        <Logo />
      </Box>
      <List>
        {Object.entries(RoutesConfig).map(([key, config]) => {
          return config.subItems ? (
            <Accordion key={key}>
              <AccordionSummary
                expandIcon={
                  <ExpandMore sx={{ color: theme.palette.primary.light }} />
                }
                aria-controls={`${config.label}-content`}
                id={`${config.label}-header`}
              >
                <ListItem key={key} disabled={config.status === "inactive"}>
                  <ListItemIcon>
                    <config.icon />
                  </ListItemIcon>
                  <ListItemText primary={config.label} />
                  {config.endAdornment && (
                    <ListItemIcon
                      sx={{ "> a": { color: theme.palette.primary.light } }}
                    >
                      {config.endAdornment}
                    </ListItemIcon>
                  )}
                </ListItem>
              </AccordionSummary>
              <AccordionDetails>
                {Object.entries(config.subItems).map(([key, config]) => (
                  <NavItem
                    router={router}
                    config={config}
                    key={key}
                    theme={theme}
                    isSubItem={true}
                  />
                ))}
              </AccordionDetails>
            </Accordion>
          ) : (
            <NavItem
              router={router}
              config={config}
              key={key}
              theme={theme}
              isSubItem={false}
            />
          );
        })}
      </List>
    </div>
  );
};

import { useTablet } from "@/hooks/responsive";
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
  useTheme
} from "@mui/material";
import dynamic from "next/dynamic";
import { NextRouter, useRouter } from "next/router";
import { Logo } from "../Atoms";
import { MenuItemType } from "../types";
import { FC } from "react";
import { Link } from "../Molecules/Link";

const MENU_ITEMS: MenuItemType[] = [
  {
    label: "Overview",
    path: "/",
    icon: dynamic(() => import("@mui/icons-material/Home")),
    status: "active",
    matches: ["/"]
  },
  {
    label: "Swap",
    path: "/swap",
    icon: dynamic(() => import("@mui/icons-material/SwapVert")),
    status: "active",
    matches: ["/swap"]
  },
  {
    label: "Pool",
    path: "/pool",
    icon: dynamic(() => import("@mui/icons-material/AttachMoney")),
    status: "active",
    matches: [
      "/pool",
      "/pool/add-liquidity",
      "/pool/select",
      "/pool/create-pool"
    ]
  },
  {
    label: "Bond",
    path: "/bond",
    icon: dynamic(() => import("@mui/icons-material/Payments")),
    status: "active",
    matches: ["/bond", "/bond/select"]
  },
  {
    label: "Staking",
    path: "/staking",
    icon: dynamic(() => import("@mui/icons-material/TollOutlined")),
    status: "active",
    matches: ["/staking"]
  },
  {
    label: "Picasso",
    path: "/picasso",
    icon: dynamic(() => import("@mui/icons-material/Autorenew")),
    status: "active",
    endAdornment: (
      <Link href="https://picasso.xyz/" target="_blank">
        <IconButton>
          <OpenInNew />
        </IconButton>
      </Link>
    )
  }
];

const MenuItem = (
  router: NextRouter,
  config: MenuItemType,
  key: string,
  theme: Theme,
  isSubItem: boolean
) => {
  const selected = config?.matches?.includes(router.pathname);
  return (
    <ListItem
      selected={selected}
      button
      onClick={() => {
        if (config.path) {
          router.push(config.path);
        }
      }}
      key={key}
      disabled={config.status === "inactive"}
      sx={{
        paddingLeft: isSubItem ? "3rem" : "1.5rem",
        marginTop: 1
      }}
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

export const NavBar: FC = () => {
  const theme = useTheme();
  const isTablet = useTablet();
  const router = useRouter();
  return (
    <Box pl={isTablet ? 0 : 4}>
      <Box
        sx={{
          padding: theme.spacing(6, 3),
          mb: theme.spacing(6)
        }}
      >
        <Logo />
      </Box>
      <List>
        {Object.entries(MENU_ITEMS).map(([key, config]) => {
          return config.subItems ? (
            <Accordion>
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
                {Object.entries(config.subItems).map(([key, config]) =>
                  MenuItem(router, config, key, theme, true)
                )}
              </AccordionDetails>
            </Accordion>
          ) : (
            MenuItem(router, config, key, theme, false)
          );
        })}
      </List>
    </Box>
  );
};

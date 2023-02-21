import ExpandMore from "@mui/icons-material/ExpandMore";
import {
  Accordion,
  AccordionDetails,
  AccordionSummary,
  Box,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  Theme,
  useTheme,
} from "@mui/material";
import { NextRouter, useRouter } from "next/router";
import { Logo } from "../Atom";
import { FC } from "react";
import config from "@/constants/config";
import { routesConfig } from "@/utils/routesConfig";

type MenuItem = {
  label: string;
  path: string;
  icon: React.ComponentType<any>;
  endAdornment?: React.ReactNode;
  status: "active" | "inactive";
  matches?: string[];
};

type NavItemProps = {
  router: NextRouter;
  config: MenuItem;
  theme: Theme;
  isSubItem: boolean;
};
const NavItem: FC<NavItemProps> = ({ router, config, theme, isSubItem }) => {
  const handleClick = () => {
    if (config.status === "active" && !config.path.startsWith("http")) {
      router.push(config.path);
    } else {
      window.open(config.path, "_blank");
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
      <ListItemIcon
        sx={{
          color: config?.matches?.includes(router.pathname)
            ? theme.palette.primary.light
            : undefined,
        }}
      >
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
  const router = useRouter();
  return (
    <div>
      <Box
        sx={{
          padding: theme.spacing(6, 3),
          mb: theme.spacing(4),
          cursor: "pointer",
        }}
      >
        <Logo />
      </Box>
      <List>
        {Object.entries(routesConfig).map(([key, config]) => {
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

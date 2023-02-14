import * as React from "react";
import { FC, ReactNode, useState } from "react";
import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import CssBaseline from "@mui/material/CssBaseline";
import Drawer from "@mui/material/Drawer";
import IconButton from "@mui/material/IconButton";
import MenuIcon from "@mui/icons-material/Menu";
import Toolbar from "@mui/material/Toolbar";
import { NavBar } from "../Molecules";
import {
  alpha,
  Breadcrumbs,
  Typography,
  useMediaQuery,
  useTheme,
} from "@mui/material";
import { PolkadotConnect } from "../Organisms/Wallet/PolkadotConnect";
import { GlobalSettings } from "../Organisms/Settings/GlobalSettings";
import { ExternalLinksDropdown } from "@/components/Molecules/ExternalLinksDropdown";
import { useConnectedEndpoint } from "@/defi/polkadot/hooks/useConnectedEndpoint";

type DefaultLayoutProps = {
  breadcrumbs?: ReactNode[];
};

function Sidebar(props: {
  drawerWidth: number;
  breadcrumbs: React.ReactNode[] | undefined;
}) {
  const theme = useTheme();
  const [mobileOpen, setMobileOpen] = useState(false);
  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen);
  };
  return (
    <>
      <AppBar
        position="fixed"
        sx={{
          width: { md: `calc(100% - ${props.drawerWidth}px)` },
          ml: { md: `${props.drawerWidth}px` },
          px: {
            xs: "0",
            md: theme.spacing(3),
          },
          backgroundColor: alpha("#0f0904", 0.9),
          backdropFilter: "blur(32x)",
        }}
      >
        <Toolbar
          sx={{
            display: "flex",
            justifyContent: "space-between",
            flexDirection: "row-reverse",
            gap: 3,
          }}
        >
          <IconButton
            color="inherit"
            aria-label="open drawer"
            edge="start"
            onClick={handleDrawerToggle}
            sx={{ mr: 2, display: { md: "none" } }}
          >
            <MenuIcon />
          </IconButton>
          <Box sx={{ display: "flex", gap: 2, alignItems: "center" }}>
            <GlobalSettings />
            <PolkadotConnect />
            <ExternalLinksDropdown />
          </Box>
          {props.breadcrumbs && (
            <Breadcrumbs separator="›" aria-label="breadcrumb">
              {props.breadcrumbs}
            </Breadcrumbs>
          )}
        </Toolbar>
      </AppBar>
      <Box
        component="nav"
        sx={{ width: { md: props.drawerWidth }, flexShrink: { md: 0 } }}
        aria-label="mailbox folders"
      >
        <Drawer
          variant="temporary"
          open={mobileOpen}
          anchor="right"
          onClose={handleDrawerToggle}
          ModalProps={{
            keepMounted: true, // Better open performance on mobile.
          }}
          sx={{
            display: {
              xs: "block",
              sm: "block",
              md: "none",
              background: theme.palette.primary.dark,
            },
            "& .MuiDrawer-paper": {
              boxSizing: "border-box",
              width: props.drawerWidth,
              padding: "0rem",
            },
          }}
        >
          <NavBar />
        </Drawer>
        <Drawer
          variant="permanent"
          sx={{
            display: { sm: "none", md: "block", xs: "none" },
            "& .MuiDrawer-paper": {
              boxSizing: "border-box",
              width: props.drawerWidth,
              padding: 0,
            },
          }}
          open
        >
          <NavBar />
        </Drawer>
      </Box>
    </>
  );
}

export const DefaultLayout: FC<DefaultLayoutProps> = (props) => {
  const { children, breadcrumbs } = props;
  const theme = useTheme();
  const isTablet = useMediaQuery(theme.breakpoints.down("md"));
  const drawerWidth = isTablet ? 240 : 320;
  const connectedEndpoint = useConnectedEndpoint();
  return (
    <Box sx={{ display: "flex", minHeight: "100vh" }}>
      <CssBaseline />
      <Sidebar drawerWidth={drawerWidth} breadcrumbs={breadcrumbs} />
      <Box
        component="main"
        sx={{
          p: 3,
          width: { sm: `calc(100% - ${drawerWidth}px)` },
          background: theme.palette.primary.dark,
          display: "flex",
          flexDirection: "column",
          justifyContent: "flex-start",
        }}
      >
        <Box
          sx={{
            display: "flex",
            alignItems: "flex-start",
            justifyContent: "flex-start",
            flexDirection: "row",
            flexGrow: 1,
          }}
        >
          {children}
        </Box>
        <Box
          sx={{
            display: "flex",
            justifyContent: "flex-end",
            alignItems: "center",
            height: theme.spacing(2),
          }}
        >
          <Typography variant="caption">{connectedEndpoint}</Typography>
        </Box>
      </Box>
    </Box>
  );
};

export default DefaultLayout;

import * as React from "react";
import { FC, ReactNode } from "react";
import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import CssBaseline from "@mui/material/CssBaseline";
import Drawer from "@mui/material/Drawer";
import IconButton from "@mui/material/IconButton";
import MenuIcon from "@mui/icons-material/Menu";
import Toolbar from "@mui/material/Toolbar";
import { NavBar } from "../Molecules";
import { alpha, Breadcrumbs, Typography, useTheme } from "@mui/material";
import { useTablet } from "@/hooks/responsive";
import { PolkadotConnect } from "../Organisms/Wallet/PolkadotConnect";
import { GlobalSettings } from "../Organisms/Settings/GlobalSettings";

type DefaultLayoutProps = {
  breadcrumbs?: ReactNode[];
};

export const DefaultLayout: FC<DefaultLayoutProps> = (props) => {
  const { children, breadcrumbs } = props;
  const [mobileOpen, setMobileOpen] = React.useState(false);
  const isTablet = useTablet();
  const theme = useTheme();
  const drawerWidth = isTablet ? 240 : 320;

  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen);
  };

  return (
    <Box sx={{ display: "flex", minHeight: "100vh" }}>
      <CssBaseline />
      <AppBar
        position="fixed"
        sx={{
          width: { md: `calc(100% - ${drawerWidth}px)` },
          ml: { md: `${drawerWidth}px` },
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
          </Box>
          {breadcrumbs && (
            <Breadcrumbs separator="›" aria-label="breadcrumb">
              {breadcrumbs}
            </Breadcrumbs>
          )}
        </Toolbar>
      </AppBar>
      <Box
        component="nav"
        sx={{ width: { md: drawerWidth }, flexShrink: { md: 0 } }}
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
              width: drawerWidth,
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
              width: drawerWidth,
              padding: 0,
            },
          }}
          open
        >
          <NavBar />
        </Drawer>
      </Box>
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
          <Typography variant="caption">
            {process.env.WEBSITE_VERSION}
          </Typography>
        </Box>
      </Box>
    </Box>
  );
};

export default DefaultLayout;

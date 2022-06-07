import { useMobile, useTablet } from "@/hooks/responsive";
import ArrowForwardIosIcon from "@mui/icons-material/ArrowForwardIos";
import MenuIcon from "@mui/icons-material/Menu";
import { Breadcrumbs, useTheme } from "@mui/material";
import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import CssBaseline from "@mui/material/CssBaseline";
import Drawer from "@mui/material/Drawer";
import IconButton from "@mui/material/IconButton";
import Toolbar from "@mui/material/Toolbar";
import * as React from "react";
import { NavBar } from "../Organisms";
import { PolkadotConnect } from "../Organisms/Wallet/PolkadotConnect";

type DefaultLayoutProps = {
  breadcrumbs?: React.ReactNode[];
  children?: React.ReactNode;
};

export const DefaultLayout: React.FC<DefaultLayoutProps> = (props) => {
  const { children, breadcrumbs } = props;

  const [mobileOpen, setMobileOpen] = React.useState(false);
  const isTablet = useTablet();
  const isMobile = useMobile();
  const theme = useTheme();
  const drawerWidth = isTablet
    ? theme.custom.drawerWidth.tablet
    : theme.custom.drawerWidth.desktop;

  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen);
  };

  return (
    <Box sx={{ display: "flex" }}>
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
            {/* TODO: adding other content for toolbar */}
            <PolkadotConnect />
          </Box>
          {!isMobile && breadcrumbs && (
            <Breadcrumbs
              separator={<ArrowForwardIosIcon sx={{ fontSize: 14 }} />}
              aria-label="breadcrumb"
            >
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
            display: { xs: "block", sm: "block", md: "none" },
            "& .MuiDrawer-paper": {
              width: drawerWidth,
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
              width: drawerWidth,
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
          flexGrow: 1,
          padding: theme.spacing(1),
          [theme.breakpoints.down("sm")]: {
            padding: theme.spacing(3),
          },
          width: { sm: `calc(100% - ${drawerWidth}px)` },
          marginTop: theme.spacing(20),
        }}
      >
        {children}
      </Box>
    </Box>
  );
};

export default DefaultLayout;

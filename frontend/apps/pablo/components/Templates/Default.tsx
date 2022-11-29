import { useMobile, useTablet } from "@/hooks/responsive";
import ArrowForwardIosIcon from "@mui/icons-material/ArrowForwardIos";
import MenuIcon from "@mui/icons-material/Menu";
import {
  Breadcrumbs,
  createTheme,
  ThemeProvider,
  Typography,
  useTheme,
} from "@mui/material";
import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import CssBaseline from "@mui/material/CssBaseline";
import Drawer from "@mui/material/Drawer";
import IconButton from "@mui/material/IconButton";
import Toolbar from "@mui/material/Toolbar";
import * as React from "react";
import { NavBar } from "../Organisms";
import { PolkadotConnect } from "../Organisms/Wallet/PolkadotConnect";
import { AnimatedCircles } from "@/components/Molecules/AnimatedCircles";
import NoSSR from "@/components/Atoms/NoSSR";
import { useMemo } from "react";

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
  const walletThemeOverride = useMemo(() => {
    return createTheme(theme, {
      components: {
        MuiDialog: {
          styleOverrides: {
            paper: {
              backgroundColor: theme.palette.background.transparentCharcoal,
              backdropFilter: "blur(32px)",
            },
            root: {
              "& .MuiBackdrop-root": {
                backdropFilter: "none",
              },
            },
          },
        },
      },
    });
  }, [theme]);
  const drawerWidth = isTablet
    ? theme.custom.drawerWidth.tablet
    : theme.custom.drawerWidth.desktop;

  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen);
  };
  return (
    <>
      <AnimatedCircles />
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
              <NoSSR>
                <ThemeProvider theme={walletThemeOverride}>
                  <PolkadotConnect />
                </ThemeProvider>
              </NoSSR>
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
            display: "flex",
            flexDirection: "column",
            padding: theme.spacing(1),
            [theme.breakpoints.down("sm")]: {
              padding: theme.spacing(3),
            },
            width: { sm: `calc(100% - ${drawerWidth}px)` },
            marginTop: theme.spacing(20),
            overflow: "hidden",
          }}
        >
          <Box sx={{ flexGrow: 1 }}>{children}</Box>
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
    </>
  );
};

export default DefaultLayout;

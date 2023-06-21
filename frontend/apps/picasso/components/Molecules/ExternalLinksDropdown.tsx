import { FC, MouseEvent, useState } from "react";
import {
  alpha,
  Box,
  Button,
  ListItemIcon,
  ListItemText,
  Menu,
  MenuItem,
  Typography,
  useTheme,
} from "@mui/material";
import { MoreHoriz, OpenInNew } from "@mui/icons-material";
import Image from "next/image";
import { Link } from "@/components";
import config from "@/constants/config";

export const ExternalLinksDropdown: FC = () => {
  const theme = useTheme();
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const open = Boolean(anchorEl);
  const handleClick = (event: MouseEvent<HTMLButtonElement>) => {
    setAnchorEl(event.currentTarget);
  };
  const handleClose = () => {
    setAnchorEl(null);
  };
  const menuItemStyles = {
    padding: theme.spacing(3),
  };

  return (
    <>
      <Button
        variant="outlined"
        color="primary"
        onClick={handleClick}
        aria-controls={open ? "basic-menu" : undefined}
        aria-haspopup="true"
        aria-expanded={open ? "true" : undefined}
      >
        <MoreHoriz />
      </Button>
      <Menu
        id="basic-menu"
        anchorEl={anchorEl}
        open={open}
        onClose={handleClose}
        anchorOrigin={{
          vertical: "bottom",
          horizontal: "right",
        }}
        transformOrigin={{
          vertical: "top",
          horizontal: "right",
        }}
        PaperProps={{
          sx: {
            padding: theme.spacing(0),
            margin: theme.spacing(2, 0, 0),
            borderRadius: theme.spacing(1.5),
            border: `1px solid ${alpha(theme.palette.common.white, 0.3)}`,
          },
        }}
        MenuListProps={{
          "aria-labelledby": "basic-button",
          sx: {
            minWidth: "285px",
            backgroundColor: theme.palette.secondary.dark,
            padding: 0,
          },
        }}
      >
        <MenuItem sx={menuItemStyles} disableRipple>
          <Link
            href={config.governanceUrl}
            target="_blank"
            sx={{
              display: "flex",
              alignItems: "center",
              justifyContent: "space-between",
              width: "100%",
            }}
          >
            <Box display="flex">
              <ListItemIcon>
                <Image
                  src={"/icons/announcement.svg"}
                  width={24}
                  height={24}
                  alt="announcement-icon"
                />
              </ListItemIcon>
              <ListItemText>Request a feature</ListItemText>
            </Box>
            <Typography
              component="div"
              color="text.secondary"
              display="flex"
              alignItems="center"
              justifyContent="center"
            >
              <OpenInNew />
            </Typography>
          </Link>
        </MenuItem>
        <MenuItem sx={menuItemStyles} disableRipple>
          <Link
            href={config.discordUrl}
            target="_blank"
            sx={{
              display: "flex",
              alignItems: "center",
              justifyContent: "space-between",
              width: "100%",
            }}
          >
            <Box display="flex">
              <ListItemIcon>
                <Image
                  src={"/icons/Discord.svg"}
                  width={24}
                  height={24}
                  alt="Discord icon"
                />
              </ListItemIcon>
              <ListItemText>Discord</ListItemText>
            </Box>
          </Link>
        </MenuItem>
        <MenuItem sx={menuItemStyles} disableRipple>
          <Link
            href={config.twitterUrl}
            target="_blank"
            sx={{
              display: "flex",
              alignItems: "center",
              justifyContent: "space-between",
              width: "100%",
            }}
          >
            <Box display="flex">
              <ListItemIcon>
                <Image
                  src={"/icons/Twitter.svg"}
                  width={24}
                  height={24}
                  alt="Twitter icon"
                />
              </ListItemIcon>
              <ListItemText>Twitter</ListItemText>
            </Box>
          </Link>
        </MenuItem>
        <MenuItem sx={menuItemStyles} disableRipple>
          <Link
            href={config.mediumUrl}
            target="_blank"
            sx={{
              display: "flex",
              alignItems: "center",
              justifyContent: "space-between",
              width: "100%",
            }}
          >
            <Box display="flex">
              <ListItemIcon>
                <Image
                  src={"/icons/Medium.svg"}
                  width={24}
                  height={24}
                  alt="Medium icon"
                />
              </ListItemIcon>
              <ListItemText>Medium</ListItemText>
            </Box>
          </Link>
        </MenuItem>
      </Menu>
    </>
  );
};

import {
  Badge,
  Box,
  IconButton,
  List,
  ListItem,
  ListItemText,
  Popover,
  useTheme,
} from "@mui/material";
import { Notifications as MuiNotificationIcon } from "@mui/icons-material";
import {
  Notification,
  useNotificationStore,
} from "@/components/Molecules/PersistentNotification/NotificationStore";

export const NotificationDrawer = () => {
  const list = useNotificationStore((state) => state.list);
  const notify = useNotificationStore((state) => state.notify);
  const isListVisible = useNotificationStore((state) => state.isListVisible);
  const toggleListVisibilty = useNotificationStore(
    (state) => state.toggleListVisibility
  );

  const theme = useTheme();

  return (
    <>
      <IconButton onClick={toggleListVisibilty}>
        <Badge
          badgeContent={Object.keys(list).length}
          max={99}
          color={"primary"}
        >
          <MuiNotificationIcon />
        </Badge>
      </IconButton>
      <Popover
        sx={{
          zIndex: 9999,
        }}
        open={isListVisible}
        anchorOrigin={{
          vertical: "top",
          horizontal: "right",
        }}
        transformOrigin={{
          vertical: "top",
          horizontal: "right",
        }}
        onClose={toggleListVisibilty}
      >
        <Box sx={{ backgroundColor: theme.palette.background.default }}>
          {Object.keys(list).length === 0 ? (
            <List>
              <ListItem>
                <ListItemText primary="No notifications found."></ListItemText>
              </ListItem>
            </List>
          ) : (
            <List>
              {Object.entries(list).map(([id, notification]) => {
                return (
                  <ListItem key={id}>
                    <ListItemText
                      primary={notification.title}
                      secondary={notification.subtitle}
                    ></ListItemText>
                  </ListItem>
                );
              })}
            </List>
          )}
        </Box>
      </Popover>
    </>
  );
};

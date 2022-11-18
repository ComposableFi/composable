import {
  Button,
  Card,
  List,
  ListItem,
  ListItemText,
  Stack,
  Typography,
} from "@mui/material";
import {
  EndpointPreset,
  endpointPresets,
  getEnvironment,
  setEndpointPreset,
} from "shared/endpoints";

export const EndpointPage = () => {
  return (
    <Stack direction="column" gap={4}>
      <Card>
        <Typography variant="h5">Current endpoints</Typography>
        <List>
          <ListItem>
            <ListItemText
              primary={"Subsquid Endpoint"}
              secondary={getEnvironment("subsquid")}
            />
          </ListItem>
          <ListItem>
            <ListItemText
              primary={"Picasso parachain"}
              secondary={getEnvironment("picasso")}
            />
          </ListItem>
          <ListItem>
            <ListItemText
              primary={"Relay chain"}
              secondary={getEnvironment("kusama")}
            />
          </ListItem>
          <ListItem>
            <ListItemText
              primary={"Karura parachain"}
              secondary={getEnvironment("karura")}
            />
          </ListItem>
          <ListItem>
            <ListItemText
              primary={"Statemine parachain"}
              secondary={getEnvironment("statemine")}
            />
          </ListItem>
        </List>
      </Card>
      <Card>
        <Typography variant="h5" mb={4}>
          Update endpoints
        </Typography>
        <Stack gap={4} direction="row">
          {Object.keys(endpointPresets).map((preset, key) => {
            return (
              <Button
                key={key}
                variant="contained"
                onClick={() => setEndpointPreset(preset as EndpointPreset)}
              >
                {preset}
              </Button>
            );
          })}
          <Button variant="outlined" onClick={() => location.reload()}>
            Apply and Refresh
          </Button>
        </Stack>
      </Card>
    </Stack>
  );
};

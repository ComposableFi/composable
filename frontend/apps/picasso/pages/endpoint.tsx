import Default from "@/components/Templates/Default";
import { List, ListItem, ListItemText } from "@mui/material";

const Endpoint = () => {
  return (
    <Default>
      <List>
        <ListItem>
          <ListItemText
            primary={process.env.SUBSQUID_URL || ""}
            secondary={"Subsquid SUBSQUID_URL"}
          />
        </ListItem>
        <ListItem>
          <ListItemText
            primary={process.env.SUBSTRATE_PROVIDER_URL_KUSAMA_2019 || ""}
            secondary={
              "Picasso(DALI) parachain SUBSTRATE_PROVIDER_URL_KUSAMA_2019"
            }
          />
        </ListItem>
        <ListItem>
          <ListItemText
            primary={process.env.SUBSTRATE_PROVIDER_URL_KUSAMA || ""}
            secondary={"Relay chain SUBSTRATE_PROVIDER_URL_KUSAMA"}
          />
        </ListItem>
        <ListItem>
          <ListItemText
            primary={process.env.SUBSTRATE_PROVIDER_URL_KARURA || ""}
            secondary={"Karura/Acala Parachain SUBSTRATE_PROVIDER_URL_KARURA"}
          />
        </ListItem>
      </List>
    </Default>
  );
};

export default Endpoint;

import { NormalText } from "@/components/Organisms/Static/NormalText";
import { List } from "../List";
import { LatinListItem } from "@/components/Organisms/Static/LatinListItem";
import { Typography } from "@mui/material";

export const CollectedInformation = () => (
  <section>
    <NormalText>
      Although at this time we do not automatically collect information from you
      when you access the Site, it is possible that we may do so in the future.
      We would utilize this information to operate and ensure the security,
      reliability, and robust performance of our Services.
    </NormalText>
    <NormalText>
      We also use tracking technologies to automatically collect information
      including, but not limited to, the following:
    </NormalText>
    <List sx={{ listStylePosition: "outside" }}>
      <LatinListItem>
        <Typography variant="body3" fontWeight="bold">
          Log Files:{" "}
        </Typography>
        Files that record events that occur in connection with your use of the
        Site. Log files are created when you view content or otherwise interact
        with the Services.
      </LatinListItem>
      <LatinListItem>
        <Typography variant="body3" fontWeight="bold">
          Cookies:{" "}
        </Typography>
        Small data files stored in your device or computer that act as a unique
        tag to identify your browser. We will only strictly use necessary
        cookies in connection with the Site and Services. For the avoidance of
        doubt, the cookies that we include are essential for you to browse the
        Site and use its features, including accessing secure areas of the Site.
        You can choose to deactivate cookies, however, in such circumstances you
        will not be able to use parts of the Services which require cookies to
        be active.
      </LatinListItem>
    </List>
    <NormalText>
      In order to improve user experience and for website optimization, and to
      facilitate our internal analysis, we may likewise: (1) store your cookie
      consent state for the current domain; (2) register data or information
      regarding any on-site behavior or actions taken; and (3) collect data or
      information from your navigation and/or interaction in the Site.
    </NormalText>
  </section>
);

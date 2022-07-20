import type { NextPage } from "next";
import React from "react";
import Default from "@/components/Templates/Default";
import { useTheme, Grid, Button, Box } from "@mui/material";
import { PageTitle, SearchInput, Select } from "@/components";

import { VotingDetailsBox } from "@/components/Molecules/VotingDetailsBox";
import HowToVoteIcon from "@mui/icons-material/HowToVote";
import CheckIcon from "@mui/icons-material/Check";
import CloseIcon from "@mui/icons-material/Close";
import BallotIcon from "@mui/icons-material/Ballot";
import { Comment } from "@mui/icons-material";
import { TabItem, Tabs } from "@/components";

const tabItems: TabItem[] = [
  {
    label: "Voting",
    icon: <HowToVoteIcon />,
  },
  {
    label: "Discussion",
    icon: <Comment />,
  },
];

const optionsTag = [
  {
    value: "tag",
    label: "Tag",
  },
];

const optionsStatus = [
  {
    value: "status",
    label: "Status",
  },
];

const Governance: NextPage = () => {
  const theme = useTheme();
  const [value, setValue] = React.useState(0);

  const handleChange = (_: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  const standardPageSize = {
    xs: 12,
  };

  return (
    <Default>
      <Grid
        container
        sx={{ mx: "auto" }}
        maxWidth={1032}
        rowSpacing={5}
        columns={10}
        direction="column"
        justifyContent="center"
      >
        <Grid item {...standardPageSize} mt={theme.spacing(9)}>
          <PageTitle title="Governance" textAlign="center" />
        </Grid>
        <Grid item {...standardPageSize}>
          <Tabs value={value} onChange={handleChange} items={tabItems} />
        </Grid>
        <Grid item {...standardPageSize}>
          <SearchInput fullWidth value="" placeholder="Search for Proposals" />
        </Grid>
        <Grid item {...standardPageSize}>
          <Box display="flex">
            <Box display="flex" width={"25%"} mr={2}>
              <Select options={optionsTag} value={optionsTag[0].value} />
            </Box>
            <Box width={"25%"} display="flex" mr={2}>
              <Select options={optionsStatus} value={optionsStatus[0].value} />
            </Box>
            <Box width={"50%"}>
              <Button fullWidth variant="contained" color="primary">
                Submit Proposal
              </Button>
            </Box>
          </Box>
        </Grid>
        <Grid item {...standardPageSize}>
          <VotingDetailsBox
            id="12"
            title="Smart Contracts on Polkadot - WASM conference (Virtual)"
            status="info"
            statusText="Active"
            timeText="19d 21h 45m remaining"
            statusIcon={<HowToVoteIcon />}
            address="12tb....432"
            tagText="Ecosystem"
            sx={{
              padding: theme.spacing(6),
            }}
          />
        </Grid>
        <Grid item {...standardPageSize}>
          <VotingDetailsBox
            id="12"
            title="Proposal Title"
            status="warning"
            statusText="Proposed"
            timeText="19d 21h 45m remaining"
            statusIcon={<BallotIcon />}
            address="12tb....432"
            tagText="Ecosystem"
            sx={{
              padding: theme.spacing(6),
            }}
          />
        </Grid>
        <Grid item {...standardPageSize}>
          <VotingDetailsBox
            id="12"
            title="Proposal Title"
            status="success"
            statusText="Passed"
            timeText="19d 21h 45m remaining"
            statusIcon={<CheckIcon />}
            address="12tb....432"
            tagText="Ecosystem"
            sx={{
              padding: theme.spacing(6),
            }}
          />
        </Grid>
        <Grid item {...standardPageSize}>
          <VotingDetailsBox
            id="12"
            title="Proposal Title"
            status="error"
            statusText="Rejected"
            timeText="19d 21h 45m remaining"
            statusIcon={<CloseIcon />}
            address="12tb....432"
            tagText="Ecosystem"
            sx={{
              padding: theme.spacing(6),
            }}
          />
        </Grid>
      </Grid>
    </Default>
  );
};

export default Governance;

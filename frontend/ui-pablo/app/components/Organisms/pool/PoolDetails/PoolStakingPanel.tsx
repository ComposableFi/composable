import { BoxProps } from "@mui/material";
import { TabItem, TabPanel, Tabs } from "@/components/Atoms";
import { BoxWrapper } from "../../BoxWrapper";
import { useState } from "react";
import { PoolStakeForm } from "./PoolStakeForm";
import { PoolUnstakeForm } from "./PoolUnstakeForm";

const tabItems: TabItem[] = [
  {
    label: "Stake",
  },
  {
    label: "Unstake",
  },
];

export const PoolStakingPanel: React.FC<BoxProps> = ({
  ...boxProps
}) => {

  const [tab, setTab] = useState<number>(0);
  const handleTabChange = (_: React.SyntheticEvent, newValue: number) => {
    setTab(newValue);
  };

  return (
    <BoxWrapper {...boxProps}>
      <Tabs items={tabItems} value={tab} onChange={handleTabChange} />
      <TabPanel index={0} value={tab}>
        <PoolStakeForm />
      </TabPanel>
      <TabPanel index={1} value={tab}>
        <PoolUnstakeForm />
      </TabPanel>
    </BoxWrapper>
  );
};


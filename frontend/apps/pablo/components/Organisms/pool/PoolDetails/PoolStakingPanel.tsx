import { TabItem, TabPanel, Tabs } from "@/components/Atoms";
import { BoxWrapper } from "../../BoxWrapper";
import { useState } from "react";
import { PoolStakeForm } from "./PoolStakeForm";
import { PoolUnstakeForm } from "./PoolUnstakeForm";
import { PoolDetailsProps } from "./index";

const tabItems: TabItem[] = [
  {
    label: "Stake",
  },
  {
    label: "Unstake",
  },
];

export const PoolStakingPanel: React.FC<PoolDetailsProps> = ({
  poolId,
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
        <PoolStakeForm poolId={poolId} />
      </TabPanel>
      <TabPanel index={1} value={tab}>
        <PoolUnstakeForm poolId={poolId} />
      </TabPanel>
    </BoxWrapper>
  );
};


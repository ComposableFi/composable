import { TabItem, TabPanel, Tabs } from "@/components/Atoms";
import { BoxWrapper } from "../../BoxWrapper";
import { FC, SyntheticEvent, useState } from "react";
import { PoolDetailsProps } from "./index";

const tabItems: TabItem[] = [
  {
    label: "Stake",
  },
  {
    label: "Unstake",
  },
];

export const PoolStakingPanel: FC<PoolDetailsProps> = ({
  pool,
  ...boxProps
}) => {
  const [tab, setTab] = useState<number>(0);
  const handleTabChange = (_: SyntheticEvent, newValue: number) => {
    setTab(newValue);
  };

  return (
    <BoxWrapper {...boxProps}>
      <Tabs items={tabItems} value={tab} onChange={handleTabChange} />
      <TabPanel index={0} value={tab}>
        {/*<PoolStakeForm pool={pool} />*/}
      </TabPanel>
      <TabPanel index={1} value={tab}>
        {/*<PoolUnstakeForm pool={pool} />*/}
      </TabPanel>
    </BoxWrapper>
  );
};

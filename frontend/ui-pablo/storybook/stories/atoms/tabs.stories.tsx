import React from "react";
import { 
  TabsProps,
  TabItem,
  Tabs, 
} from "@/components";
import { 
  Box, 
  SxProps,
} from "@mui/material";
import Image from "next/image";
import { Story } from "@storybook/react";

const TabsStories = (props: TabsProps) => {
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 4,
    resize: "both",
    overflow: "auto",
  };

  const [value, setValue] = React.useState(0);

  const handleChange = (_: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  return (
    <Box sx={boxStyle}>
      <Tabs {...props} value={value} onChange={handleChange} />
    </Box>
  );
};
export default {
  title: "atoms/Tabs",
  component: Tabs,
};

const textOnlyItems: TabItem[] = [
  {
    label: 'Tab 1',
  },
  {
    label: 'Tab 2',
  },
  {
    label: 'Tab 3',
  }
];

const iconItems: TabItem[] = [
  {
    label: 'Tab 1',
    icon: <Image src="/dummy/network.svg" alt="Arbitrum" width={24} height={24} />,
  },
  {
    label: 'Tab 2',
    icon: <Image src="/dummy/network.svg" alt="Arbitrum" width={24} height={24} />,
  },
  {
    label: 'Tab 3',
    icon: <Image src="/dummy/network.svg" alt="Arbitrum" width={24} height={24} />,
  }
]

const Template: Story<typeof TabsStories> = (args) => (
  <TabsStories {...args} />
);

export const TextOnlyTabs = Template.bind({});
TextOnlyTabs.args = {
  items: textOnlyItems,
}

export const IconTabs = Template.bind({});
IconTabs.args = {
  items: iconItems,
}

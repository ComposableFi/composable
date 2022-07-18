import {
    AMMAsset
  } from "@/components";
  import { ComponentMeta, ComponentStory } from "@storybook/react";
  import { AMM_IDS } from "@/defi/AMMs";
 
  
  export default {
    title: "atoms/AMMAsset",
    component: AMMAsset,
    argTypes: {
      id: {
        options: AMM_IDS,
        control: { type: "select" },
      },
    }
  } as ComponentMeta<typeof AMMAsset>;
  
  const Template: ComponentStory<typeof AMMAsset> = (args) => <AMMAsset {...args} />;


  export const Default = Template.bind({});

  Default.args = {
    id: AMM_IDS[0]
  }
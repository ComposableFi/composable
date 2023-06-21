import React from 'react';
import {
  TabsProps as MuiTabsProps, 
  Tabs as MuiTabs,
  Tab as MuiTab,
} from "@mui/material";

export type TabItem = {
  label: string,
  icon?: JSX.Element,
  disabled?: boolean,
};

export type TabsProps = {
  items?: TabItem[],
  value?: number,
  onChange?: (_: React.SyntheticEvent, newValue: number) => any,
} & MuiTabsProps;

export const Tabs: React.FC<TabsProps> = ({
  items,
  value,
  onChange,
  ...rest
}) => {

  return (
    <MuiTabs 
      value={value} 
      onChange={onChange} 
      variant='fullWidth'
      {...rest}
    >
      {
        items && items.map((item, index) => (
          <MuiTab
            key={index} 
            label={item.label} 
            icon={item.icon} 
            iconPosition="start" 
            disabled={item.disabled} />
        ))
      }
    </MuiTabs>
  );
  
};

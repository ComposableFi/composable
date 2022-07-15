import { AlertColor } from "@mui/material";

export type Asset = {
  icon: string;
  label?: string;
};

type MenuItem = {
  label: string;
  path: string;
  icon: React.ComponentType<any>;
  endAdornment?: React.ReactNode;
  status: "active" | "inactive";
  matches?: string[];
};

export type MenuItemType = MenuItem & {
  subItems?: Array<MenuItem>;
};

export type Message = {
  title?: string,
  text?: string,
  link?: string,
  severity?: AlertColor,
};

export type Option = {
  value: any;
  label: string;
  shortLabel?: string;
  icon?: string;
  disabled?: boolean;
  hidden?: boolean;
};

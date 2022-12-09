import { Period } from "./utils";
import { Mark } from "@mui/base";
export interface DropdownOption {
    value: string;
    label: string;
    shortLabel?: string;
}

export interface DropdownOptionWithIcon extends DropdownOption {
    icon: string;
}

export interface DurationPresetMark extends Mark {
    period: Period;
    periodInSeconds: number;
    periodInString: string;
    value: number;
}
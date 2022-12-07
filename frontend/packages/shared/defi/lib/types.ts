export interface DropdownOption {
    value: string;
    label: string;
    shortLabel?: string;
}

export interface DropdownOptionWithIcon extends DropdownOption {
    icon: string;
}
import {
  Accordion,
  AccordionDetails,
  AccordionSummary,
  Typography,
} from "@mui/material";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import React, { useEffect, useState } from "react";
import { Option, Select } from "../Atom";

const headerStyle = {
  "&.MuiAccordionSummary-root": {
    paddingRight: 0,
  },

  "&.Mui-expanded": {
    minHeight: "3rem",

    "& > .MuiAccordionSummary-content.Mui-expanded": {
      margin: 0,
    },
  },
};

const expandIconStyle = {
  color: "primary.light",
};

const detailsStyle = {
  "&.MuiAccordionDetails-root": {
    marginTop: "1rem",
    marginBottom: "0.5rem",
  },
};

export type RecipientDropDownProps = {
  value: string;
  expanded: boolean;
  options: Array<{ value: string, label: string, icon: string }>;
  setValue?: React.Dispatch<React.SetStateAction<any>>;
};

export const RecipientDropdown: React.FC<RecipientDropDownProps> = ({
  value,
  expanded,
  options,
  setValue,
}) => {
  const [isExpanded, setIsExpanded] = useState(expanded);

  useEffect(() => setIsExpanded(expanded), [expanded]);

  const handleChange = (_: React.SyntheticEvent, newExpanded: boolean) => {
    setIsExpanded(newExpanded);
  };

  return (
    <Accordion expanded={isExpanded} onChange={handleChange}>
      <AccordionSummary
        sx={headerStyle}
        expandIcon={<ExpandMoreIcon sx={expandIconStyle} />}
        aria-controls="recipient-content"
        id="recipient-header"
      >
        <Typography variant="body2" color="primary.light">
          Recipient
        </Typography>
      </AccordionSummary>
      <AccordionDetails sx={detailsStyle}>
        <Select
          value={value}
          searchable={true}
          centeredLabel={true}
          options={options}
          setValue={setValue}
        />
      </AccordionDetails>
    </Accordion>
  );
};

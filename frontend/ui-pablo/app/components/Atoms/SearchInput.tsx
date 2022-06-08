import React from 'react';
import { 
  Button, 
  InputAdornment, 
  TextField,
  TextFieldProps, 
  useTheme,
} from "@mui/material";
import SearchIcon from '@mui/icons-material/Search';
import CloseIcon from '@mui/icons-material/Close';

export type SearchInputProps = {
  setValue?: React.Dispatch<React.SetStateAction<any>>,
  noBorder?: boolean,
} & TextFieldProps;

export const SearchInput: React.FC<SearchInputProps> = ({
  setValue,
  value,
  disabled,
  noBorder = true,
  ...rest
}) => {
  const theme = useTheme();
  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setValue && setValue(event.target.value);
  }

  const clearHandler = () => {
    setValue && setValue("");
  }

  return (
    <TextField
      value={value}
      disabled={disabled}
      onChange={handleChange}
      InputProps={{
        startAdornment: (
          <InputAdornment 
            position='start'
            sx={{
              marginRight: 0,
            }}
          >
        
            <SearchIcon />
          </InputAdornment>
        ),
        endAdornment: (
          value && !disabled ? 
          <Button
            size="small"
            onClick={clearHandler}
            variant="text"
            sx={{
              padding: theme.spacing(0.25),
              marginRight: theme.spacing(1),
            }}
          >
            <CloseIcon />
          </Button>  
          : <></>
        ),   
      }}
      sx={{
        "& .MuiOutlinedInput-notchedOutline": {
          borderWidth: noBorder ? 0 : undefined,
        },
      }}
      {...rest}
    />
  );
  
};

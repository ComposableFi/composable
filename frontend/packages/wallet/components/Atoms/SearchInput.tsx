import {
  Button,
  InputAdornment,
  TextField,
  TextFieldProps,
} from "@mui/material";
import SearchIcon from "@mui/icons-material/Search";
import CloseIcon from "@mui/icons-material/Close";

export type SearchInputProps = {
  setValue?: React.Dispatch<React.SetStateAction<any>>;
} & TextFieldProps;

export const SearchInput: React.FC<SearchInputProps> = ({
  setValue,
  value,
  disabled,
  ...rest
}) => {
  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setValue && setValue(event.target.value);
  };

  const clearHandler = () => {
    setValue && setValue("");
  };

  return (
    <TextField
      value={value}
      disabled={disabled}
      onChange={handleChange}
      InputProps={{
        startAdornment: (
          <InputAdornment
            position="start"
            sx={{
              marginRight: 0,
            }}
          >
            <SearchIcon />
          </InputAdornment>
        ),
        endAdornment:
          value && !disabled ? (
            <Button onClick={clearHandler} variant="text">
              <CloseIcon />
            </Button>
          ) : (
            <></>
          ),
      }}
      {...rest}
    />
  );
};

import { DUMMY_LAUNCH_DESCRIPTION } from "@/defi/utils";
import { 
  Box, 
  BoxProps, 
  Typography, 
} from "@mui/material";

export type LaunchDescriptionProps = BoxProps;

export const LaunchDescription: React.FC<LaunchDescriptionProps> = ({
  ...rest
}) => {
  return (
    <Box {...rest}>
      <Typography variant="h6">
        Launch description
      </Typography>
      {
        DUMMY_LAUNCH_DESCRIPTION().map((paragraph, index) => (
          <Typography variant="subtitle1" color="text.secondary" mt={4} key={index}>
            {paragraph}
          </Typography>
        ))
      }
    </Box>
  );
}
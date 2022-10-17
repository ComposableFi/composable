import { BoxProps, Typography } from "@mui/material";
import { HighlightBox } from "@/components/Atoms/HighlightBox";

export type BoxWrapperProps = {
  title?: string;
} & BoxProps;

export const BoxWrapper: React.FC<BoxWrapperProps> = ({
  title,
  children,
  ...boxProps
}) => {
  return (
    <HighlightBox p={4} {...boxProps} textAlign="left">
      {title && (
        <Typography variant="h6" mb={4}>
          {title}
        </Typography>
      )}
      {children}
    </HighlightBox>
  );
};

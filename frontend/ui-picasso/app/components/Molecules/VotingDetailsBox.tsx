import React from "react";
import {
  Box,
  Grid,
  Chip,
  Typography,
  useMediaQuery,
  useTheme,
  alpha,
  GridProps,
} from "@mui/material";
import { AccessTime, Comment } from "@mui/icons-material";

export type VotingDetailsBoxProps = {
  id?: string;
  title?: string;
  status?:
    | "default"
    | "primary"
    | "secondary"
    | "error"
    | "info"
    | "success"
    | "warning";
  statusText?: string;
  timeText?: string;
  statusIcon?: JSX.Element;
  address?: string;
  tagText?: string;
} & GridProps;

export const VotingDetailsBox: React.FC<VotingDetailsBoxProps> = ({
  id,
  title,
  status,
  statusText,
  timeText,
  statusIcon,
  address,
  tagText,
  ...rest
}) => {
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down("sm"));

  return (
    <Grid
      padding={isMobile ? [3, 2] : 2}
      display="flex"
      alignItems="flex-start"
      justifyContent="space-between"
      bgcolor={alpha(theme.palette.primary.light, 0.05)}
      borderRadius={isMobile ? undefined : 1}
      container
      {...rest}
    >
      <Grid item>
        <Typography
          variant="subtitle1"
          color="text.primary"
          mb={2}
          component="div"
        >
          #{id}
        </Typography>
      </Grid>
      <Grid item xs={9.3}>
        <Box display="flex" justifyContent={"space-between"}>
          <Box>
            <Box display="flex">
              <Typography
                variant="subtitle1"
                color="text.primary"
                mb={1}
                mr={2}
                component="div"
              >
                {title}
              </Typography>
              <Box mr={1}>
                <Chip label={tagText} />
              </Box>
            </Box>
            <Box mb={1}>
              <Typography
                variant="caption"
                color="text.secondary"
                component="span"
                mr={1}
              >
                by {address}
              </Typography>
              <Typography
                variant="caption"
                color="text.secondary"
                component="span"
                mr={1}
                p={0.4}
                borderRadius={0.5}
                bgcolor={alpha(theme.palette.primary.main, 0.3)}
              >
                Team
              </Typography>
            </Box>
            <Box display="flex" alignItems="center">
              <AccessTime
                sx={{
                  color: "text.secondary",
                  fontSize: 13,
                }}
              />
              <Typography
                variant="caption"
                color="text.secondary"
                component="div"
                ml={1}
                mr={2}
              >
                {timeText}
              </Typography>
              <Comment
                sx={{ color: theme.palette.text.secondary, fontSize: 13 }}
              />
              <Typography
                variant="caption"
                color="text.secondary"
                component="span"
                mr={1}
                ml={1}
              >
                No comments
              </Typography>
            </Box>
          </Box>
          <Box>
            {statusIcon ? (
              <Chip
                color={status ?? "default"}
                label={statusText ?? ""}
                icon={statusIcon}
              />
            ) : (
              <Chip color={status ?? "default"} label={statusText ?? ""} />
            )}
          </Box>
        </Box>
      </Grid>
    </Grid>
  );
};

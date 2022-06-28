import NextLink from "next/link";
import { Link as MuiLink, LinkProps } from "@mui/material";

export const Link: React.FC<LinkProps> = ({ children, ...props }) => (
  <NextLink href={props.href || "/"} passHref>
    <MuiLink underline="none" variant="body2" display="flex" {...props}>
      {children}
    </MuiLink>
  </NextLink>
);

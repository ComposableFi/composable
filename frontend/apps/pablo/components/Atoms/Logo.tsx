import React from "react";
import { useMediaQuery, useTheme } from "@mui/material";
import Image from "next/image";
import { useRouter } from "next/router";
import { Link } from "../Molecules";

export const Logo: React.FC = () => {
  const router = useRouter();
  const theme = useTheme();
  return (
    <Link href="/overview">
      <Image
        src="/logos/pablo.svg"
        alt="logo"
        width="130"
        height="40"
      />
    </Link>
  );
};

import React from "react";
import Image from "next/image";
import { useRouter } from "next/router";
import { Link } from "../Molecules";

export const Logo: React.FC = () => {
  const router = useRouter();

  return (
    <Link onClick={() => router.push("/")}>
      <Image
        src="/logos/pablo.svg"
        alt="logo"
        width="130"
        height="40"
      />
    </Link>
  );
};

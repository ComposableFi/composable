import { PageTitle } from "@/components";
import React from "react";
import { UnavailableFeature } from "@/components/Molecules/UnavailableFeature";

export const Header = () => (
  <>
    <PageTitle
      title="Transfers"
      subtitle="You will be able to move assets across available Kusama chains."
      textAlign="center"
    />
    <UnavailableFeature pageTitle={"Transfers"} />
  </>
);

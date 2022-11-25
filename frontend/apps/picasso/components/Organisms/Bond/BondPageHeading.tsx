import { PageTitle } from "@/components";
import { UnavailableFeature } from "@/components/Molecules/UnavailableFeature";

export const BondPageHeading = () => {
  return (
    <>
      <PageTitle
        title="Bond"
        subtitle="Bond PICA for CHAOS"
        textAlign="center"
      />
      <UnavailableFeature pageTitle={"Bonds"} />
    </>
  );
};

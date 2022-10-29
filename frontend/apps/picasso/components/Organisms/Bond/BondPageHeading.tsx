import { PageTitle } from "@/components";
import { UnavailableFeature } from "@/components/Molecules/UnavailableFeature";

export const BondPageHeading = () => {
  return (
    <>
      <PageTitle
        title="Bond"
        subtitle="Something about earning PICA"
        textAlign="center"
      />
      <UnavailableFeature pageTitle={"Bonds"} />
    </>
  );
};

import { PageTitle } from "@/components";
import { UnavailableFeature } from "@/components/Molecules/UnavailableFeature";

export const BondPageHeading = () => {
  return (
    <>
      <PageTitle
        title="Bond"
        subtitle="Provide liquidity on Picasso to earn PICA"
        textAlign="center"
      />
      <UnavailableFeature pageTitle={"Bonds"} />
    </>
  );
};

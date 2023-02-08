import { DefaultLayout, PageTitle } from "@/components";
import { Grid } from "@mui/material";
import { Preface } from "@/components/Organisms/Static/TermsOfUse/Preface";
import { List } from "@/components/Organisms/Static/List";
import { ListItem } from "@/components/Organisms/Static/ListItem";
import { Eligibility } from "@/components/Organisms/Static/TermsOfUse/Eligibility";
import { InformationalResource } from "@/components/Organisms/Static/TermsOfUse/InformationalResource";
import { IntellectualPropertyRights } from "@/components/Organisms/Static/TermsOfUse/IntellectualPropertyRights";
import { ThirdPartyWebsiteAndContent } from "@/components/Organisms/Static/TermsOfUse/ThirdPartyWebsiteAndContent";
import { UnacceptableUseOrConduct } from "@/components/Organisms/Static/TermsOfUse/UnacceptableUseOrConduct";
import { ForwardLookingStatements } from "@/components/Organisms/Static/TermsOfUse/ForwardLookingStatements";
import { UseCases } from "@/components/Organisms/Static/TermsOfUse/UseCases";
import { NotAnOffering } from "@/components/Organisms/Static/TermsOfUse/NotAnOffering";
import { NotProfessionalAdvice } from "@/components/Organisms/Static/TermsOfUse/NotProfessionalAdvice";
import { AssumptionOfRisks } from "@/components/Organisms/Static/TermsOfUse/AssumptionOfRisks";
import { NoWarranties } from "@/components/Organisms/Static/TermsOfUse/NoWarranties";
import { Staking } from "@/components/Organisms/Static/TermsOfUse/Staking";
import { LimitationOfLiability } from "@/components/Organisms/Static/TermsOfUse/LimitationOfLiability";
import { IndemnificationAndFullRelease } from "@/components/Organisms/Static/TermsOfUse/IndemnificationAndFullRelease";
import { DisputeResolution } from "@/components/Organisms/Static/TermsOfUse/DisputeResolution";
import { Indemnification } from "@/components/Organisms/Static/TermsOfUse/Indemnification";
import { ReservedRights } from "@/components/Organisms/Static/TermsOfUse/ReservedRights";
import { Assignment } from "@/components/Organisms/Static/TermsOfUse/Assignment";
import { GoverningLawAndVenue } from "@/components/Organisms/Static/TermsOfUse/GoverningLawAndVenue";
import { AccessAndAcceptance } from "@/components/Organisms/Static/TermsOfUse/AccessAndAcceptance";
import { EntireAgreement } from "@/components/Organisms/Static/TermsOfUse/EntireAgreement";

export default function TermsOfUse() {
  return (
    <DefaultLayout>
      <Grid
        container
        sx={{ mx: "auto", textAlign: "justify" }}
        maxWidth="58ch"
        rowSpacing={5}
        columns={10}
        direction="column"
        justifyContent="center"
        gap={4}
      >
        <Grid item xs={12} mt={9}>
          <article>
            <header>
              <PageTitle title={"Terms of use"} subtitle="January 17, 2023" />
            </header>
            <Preface />
            <List
              sx={{
                listStylePosition: "inside",
                padding: 0,
              }}
            >
              <ListItem>Eligibility</ListItem>
              <Eligibility />
              <ListItem>Informational Resource</ListItem>
              <InformationalResource />
              <ListItem>Intellectual Property Rights</ListItem>
              <IntellectualPropertyRights />
              <ListItem>Third-Party Website and Content</ListItem>
              <ThirdPartyWebsiteAndContent />
              <ListItem>Unacceptable Use or Conduct</ListItem>
              <UnacceptableUseOrConduct />
              <ListItem>Forward-Looking Statements</ListItem>
              <ForwardLookingStatements />
              <ListItem>Use Cases</ListItem>
              <UseCases />
              <ListItem>Not an Offering</ListItem>
              <NotAnOffering />
              <ListItem>Not Professional Advice</ListItem>
              <NotProfessionalAdvice />
              <ListItem>Assumption of Risks</ListItem>
              <AssumptionOfRisks />
              <ListItem>No Warranties</ListItem>
              <NoWarranties />
              <ListItem>Staking</ListItem>
              <Staking />
              <ListItem>Limitation of Liability</ListItem>
              <LimitationOfLiability />
              <ListItem>Indemnification and Full Release</ListItem>
              <IndemnificationAndFullRelease />
              <ListItem>Dispute Resolution</ListItem>
              <DisputeResolution />
              <ListItem>Indemnification</ListItem>
              <Indemnification />
              <ListItem>Reserved Rights</ListItem>
              <ReservedRights />
              <ListItem>Assignment</ListItem>
              <Assignment />
              <ListItem>Governing Law and Venue</ListItem>
              <GoverningLawAndVenue />
              <ListItem>Entire Agreement</ListItem>
              <EntireAgreement />
              <ListItem>Access and Acceptance</ListItem>
              <AccessAndAcceptance />
            </List>
          </article>
        </Grid>
      </Grid>
    </DefaultLayout>
  );
}

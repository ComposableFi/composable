import { DefaultLayout, PageTitle } from "@/components";
import { Preface } from "@/components/Organisms/Static/PrivacyPolicy/Preface";
import { Grid } from "@mui/material";
import { List } from "@/components/Organisms/Static/List";
import React from "react";
import { ListItem } from "@/components/Organisms/Static/ListItem";
import { PersonalDataCollected } from "@/components/Organisms/Static/PrivacyPolicy/PersonalDataCollected";
import { CollectedInformation } from "@/components/Organisms/Static/PrivacyPolicy/CollectedInformation";
import { UseOfPersonalData } from "@/components/Organisms/Static/PrivacyPolicy/UseOfPersonalData";
import { SharingAndDisclosureOfInformation } from "@/components/Organisms/Static/PrivacyPolicy/SharingAndDisclosureOfInformation";
import { OtherParties } from "@/components/Organisms/Static/PrivacyPolicy/OtherParties";
import { OtherSources } from "@/components/Organisms/Static/PrivacyPolicy/OtherSources";
import { DataSecurity } from "@/components/Organisms/Static/PrivacyPolicy/DataSecurity";
import { InternationalTransfer } from "@/components/Organisms/Static/PrivacyPolicy/InternationalTransfer";
import { YourRights } from "@/components/Organisms/Static/PrivacyPolicy/YourRights";
import { RetentionPeriod } from "@/components/Organisms/Static/PrivacyPolicy/RetentionPeriod";
import { RefusalAndNonConsent } from "@/components/Organisms/Static/PrivacyPolicy/RefusalAndNonConsent";
import { WithdrawalAndDeactivation } from "@/components/Organisms/Static/PrivacyPolicy/WithdrawalAndDeactivation";
import { LegalAge } from "@/components/Organisms/Static/PrivacyPolicy/LegalAge";
import { ApplicableLawAndJurisdiction } from "@/components/Organisms/Static/PrivacyPolicy/ApplicableLawAndJurisdiction";
import { SpecificDisclosuresAndNotices } from "@/components/Organisms/Static/PrivacyPolicy/SpecificDisclosuresAndNotices";
import { ContactUs } from "@/components/Organisms/Static/PrivacyPolicy/ContactUs";

export default function PrivacyPolicy() {
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
              <PageTitle
                title={"Privacy policy"}
                subtitle="(As of November 28, 2022)"
              />
            </header>
            <Preface />
            <List
              sx={{
                listStylePosition: "inside",
                padding: 0,
              }}
            >
              <ListItem>Personal Data Collected by the Company. </ListItem>
              <PersonalDataCollected />
              <ListItem>Collected Information</ListItem>
              <CollectedInformation />
              <ListItem>Use of Personal Data</ListItem>
              <UseOfPersonalData />
              <ListItem>Sharing and Disclosure of Information</ListItem>
              <SharingAndDisclosureOfInformation />
              <ListItem>Other Parties</ListItem>
              <OtherParties />
              <ListItem>Other Sources</ListItem>
              <OtherSources />
              <ListItem>Data Security</ListItem>
              <DataSecurity />
              <ListItem>International Transfer</ListItem>
              <InternationalTransfer />
              <ListItem>Your Rights</ListItem>
              <YourRights />
              <ListItem>Retention Period</ListItem>
              <RetentionPeriod />
              <ListItem>Refusal and Non-Consent</ListItem>
              <RefusalAndNonConsent />
              <ListItem>Withdrawal and Deactivation</ListItem>
              <WithdrawalAndDeactivation />
              <ListItem>Legal Age</ListItem>
              <LegalAge />
              <ListItem>Applicable Law and Jurisdiction</ListItem>
              <ApplicableLawAndJurisdiction />
              <ListItem>Specific Disclosures and Notices</ListItem>
              <SpecificDisclosuresAndNotices />
              <ListItem>Contact Us</ListItem>
              <ContactUs />
            </List>
          </article>
        </Grid>
      </Grid>
    </DefaultLayout>
  );
}

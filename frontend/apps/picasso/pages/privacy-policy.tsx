import { DefaultLayout, Link, PageTitle } from "@/components";
import { Grid, Typography } from "@mui/material";
import { List } from "@/components/Organisms/Static/List";
import { ListItem } from "@/components/Organisms/Static/ListItem";
import { NormalText } from "@/components/Organisms/Static/NormalText";
import { LatinListItem } from "@/components/Organisms/Static/LatinListItem";
import { DiscListItem } from "@/components/Organisms/Static/DiscListItem";

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
const ApplicableLawAndJurisdiction = () => (
  <section>
    <NormalText>
      This Privacy Policy shall be governed by and construed in accordance with
      the laws of Saint Lucia, without regard to conflict of law principles. Any
      disputes arising in respect of this Privacy Policy shall be submitted to
      the exclusive jurisdiction of the courts of Saint Lucia.
    </NormalText>
  </section>
);

const CollectedInformation = () => (
  <section>
    <NormalText>
      Although at this time we do not automatically collect information from you
      when you access the Site, it is possible that we may do so in the future.
      We would utilize this information to operate and ensure the security,
      reliability, and robust performance of our Services.
    </NormalText>
    <NormalText>
      We also use tracking technologies to automatically collect information
      including, but not limited to, the following:
    </NormalText>
    <List sx={{ listStylePosition: "outside" }}>
      <LatinListItem>
        <Typography variant="body3" fontWeight="bold">
          Log Files:{" "}
        </Typography>
        Files that record events that occur in connection with your use of the
        Site. Log files are created when you view content or otherwise interact
        with the Services.
      </LatinListItem>
      <LatinListItem>
        <Typography variant="body3" fontWeight="bold">
          Cookies:{" "}
        </Typography>
        Small data files stored in your device or computer that act as a unique
        tag to identify your browser. We will only strictly use necessary
        cookies in connection with the Site and Services. For the avoidance of
        doubt, the cookies that we include are essential for you to browse the
        Site and use its features, including accessing secure areas of the Site.
        You can choose to deactivate cookies, however, in such circumstances you
        will not be able to use parts of the Services which require cookies to
        be active.
      </LatinListItem>
    </List>
    <NormalText>
      In order to improve user experience and for website optimization, and to
      facilitate our internal analysis, we may likewise: (1) store your cookie
      consent state for the current domain; (2) register data or information
      regarding any on-site behavior or actions taken; and (3) collect data or
      information from your navigation and/or interaction in the Site.
    </NormalText>
  </section>
);

const ContactUs = () => (
  <section>
    <NormalText>
      If you have any requests pursuant to the above provisions, questions or
      comments about this Privacy Policy, our data practices, or our compliance
      with applicable law, please contact us by email at:
      <Link variant="body3" href="mailto:legal@composable.finance">
        legal@composable.finance
      </Link>{" "}
      or by mail at: 1st Floor, The Sotheby Building, Rodney Village, Rodney
      Bay, LC 04 101, Gros Islet, Saint Lucia.
    </NormalText>
  </section>
);

const DataSecurity = () => (
  <section>
    <NormalText>
      We implement and maintain reasonable administrative, physical, and
      technical security safeguards to help protect your Personal Data from
      outside attacks or threats, loss, theft, misuse, unauthorized access,
      disclosure, alteration, and destruction. Nevertheless, we do not provide
      any guarantee regarding the effectiveness of these protective measures or
      our ability to prevent other parties, acting unlawfully, from gaining
      access to your Personal Data.
    </NormalText>
  </section>
);

const InternationalTransfer = () => (
  <section>
    Information collected through the Services may be transferred to, processed,
    stored, and used in the European Economic Area (“EEA”), the UK, and other
    jurisdictions. Data protection laws in the EEA, the UK and other
    jurisdictions may be different from the laws of your country of residence.
    Your use of the Services or provision of any information therefore
    constitutes your consent to the transfer to and from, processing, usage,
    sharing, and storage of your Personal Data in the EEA, the UK and other
    jurisdictions as set out in this Privacy Policy.
  </section>
);

const LegalAge = () => (
  <section>
    <NormalText>
      The Services are intended for general audiences and are directed to users
      who are of legal age. Minors who lack the legal capacity under any
      applicable laws are generally not permitted to access or use our Services.
      Any person who becomes aware of or believes that a minor is using our
      Services should immediately contact us. We shall remove any information
      collected from the minor who may have used our Services without our
      knowledge to ensure compliance with applicable laws.
    </NormalText>
  </section>
);

const OtherParties = () => (
  <section>
    <NormalText>
      We may integrate technologies operated or controlled by other parties into
      parts of the Services. For example, the Services may include links that
      hyperlink to websites, platforms, and other services not operated or
      controlled by us.
    </NormalText>
    <NormalText>
      When you interact with other parties, including when you leave the Site,
      the other parties may independently collect information about you and
      solicit information from you. The information collected and stored by
      other parties remains subject to their own policies and practices,
      including the information they share with us, your rights and choices on
      their services and devices, and whether they store information anywhere in
      the world. We do not control such other parties or any of their content.
      We shall not be held liable or be responsible for the contents and privacy
      policies of or services provided by the other parties. We encourage you to
      read and familiarize yourself with their privacy policies and terms of
      use.
    </NormalText>
  </section>
);

const OtherSources = () => (
  <section>
    <NormalText>
      We do not collect Personal Data relating to you from other sources. If we
      do, we will inform you in advance by contacting you through the contact
      information you have provided.
    </NormalText>
  </section>
);

const PersonalDataCollected = () => (
  <section>
    <NormalText>
      When you access any of the Services, the Company may collect the following
      categories of data from you or about you which may be considered as
      Personal Data as defined in Part 1(2) of the PDPA and other privacy laws,
      rules, and regulations as they may become applicable (the “Personal Data”
      or “Information”):
    </NormalText>
    <List sx={{ listStylePosition: "outside" }}>
      <LatinListItem>
        Contact information, such as name, email address, telephone and/or
        mobile phone number
      </LatinListItem>
      <LatinListItem>
        Identification information, such as nationality, identification number
        and date of birth
      </LatinListItem>
      <LatinListItem>
        Image information, such as selfie, photo of national ID card
      </LatinListItem>
      <LatinListItem>
        Transaction information relating to your use of the Services including,
        without limitation, your transaction(s) with other users, deposit and
        withdrawal
      </LatinListItem>
      <LatinListItem>
        Device information, in particular as to the computer, device, or browser
        that you rely on in order to access the Site or use the Services,
        operating system name and version, device manufacturer and model,
        language, internet browser type and version, and the name and version of
        the Services you are using
      </LatinListItem>
      <LatinListItem>
        Your IP address: The Company automatically logs your IP address (a
        number that is assigned to the computer that you use to access the
        internet) in its server log files whenever you access the Site or use
        the Services, along with the time of your visit and the page(s) that you
        visited.
      </LatinListItem>
      <LatinListItem>
        Account information, information that is generated by your account
        activity including, but not limited to, trading activity, order
        activity, deposit, withdrawal, and account balances
      </LatinListItem>
      <LatinListItem>
        Correspondence, information that you provide, including those we have
        not solicited from you (in such a case, you are solely responsible for
        such information) to use in correspondence, including during the opening
        the account, and ongoing customer support
      </LatinListItem>
      <LatinListItem>
        Publicly-available blockchain data such as your blockchain wallet
        address, as applicable
      </LatinListItem>
      <LatinListItem>
        Additional information that we may have requested to ensure
        implementation of any contractual obligations or in connection with the
        exercise of your right(s) under this Privacy Policy
      </LatinListItem>
    </List>
    <NormalText>
      In addition to the foregoing, you may also be required to provide
      additional Personal Data when you, among others, require support, send us
      information for troubleshooting or other analysis, participate in a survey
      or other promotional activities of the Company or solicit any other
      communication from the Company.
    </NormalText>
    <NormalText>
      You may decide whether or not to submit any Personal Data required by the
      Company. You should, however, ensure that all Personal Data submitted to
      us is complete, accurate, true and correct. Failure on your part to do so
      may result in our inability to provide you with the Services that you have
      requested.
    </NormalText>
  </section>
);

const Preface = () => (
  <section>
    <NormalText>
      This Privacy Policy, among others, seeks to notify and inform you as a
      user (“you”, “your”, or “user”) of how Composable Finance Ltd.
      (“Composable”, “we”, “our”, “us” or the “Company”) collects, uses, manages
      and shares Personal Data (as hereinafter defined) in our websites at{" "}
      <Link
        variant="body3"
        href="https://www.composable.finance/"
        target="_blank"
      >
        https://www.composable.finance/
      </Link>
      ,{" "}
      <Link variant="body3" href="https://picasso.xyz">
        picasso.xyz
      </Link>
      ,{" "}
      <Link variant="body3" href="/">
        app.picasso.xyz
      </Link>
      , and such other websites, web apps, or online location that links to this
      Privacy Policy (collectively, the “Site”) and in connection with the
      services provided by the Company or through a related affiliate in the
      Site (collectively, the “Services”), as well as your rights and choices
      regarding such Personal Data, in compliance with Saint Lucia’s Privacy and
      Data Protection Act (“PDPA”), the United Kingdom’s (“UK”) Data Protection
      Act, the European Union’s General Data Privacy Regulation (“GDPR”) and
      other privacy laws, rules, and regulations as they may become applicable.
      This Privacy Policy applies to Personal Data collected, used, stored,
      disclosed and/or processed by the Company.
    </NormalText>
    <NormalText>
      By submitting information to us, or signing for the Services offered by
      us, you agree and consent to the collection, use, retention and disclosure
      by the Company and any of its affiliates, representatives, agents,
      advisors and/or service providers, as well as any other activities
      described in this Privacy Policy, of your Personal Data and other
      information. If you do not agree with the terms of this Privacy Policy,
      you should immediately discontinue the use of the Services and refrain
      from accessing the Site.
    </NormalText>
    <NormalText>
      We reserve the right to revise and update this Privacy Policy at any time.
      Any changes will be effective immediately upon our posting of the latest
      version of the Privacy Policy in the Site. Your continued use of the
      Services indicates your consent to the latest version of the Privacy
      Policy then published or posted.
    </NormalText>
  </section>
);

const RefusalAndNonConsent = () => (
  <section>
    <NormalText>
      If you refuse to provide your Personal Data or to consent to the transfer
      thereof in accordance with this Privacy Policy, we may not be able to
      fulfill our contractual requirements or, in certain cases, may not be able
      to continue providing our Services.
    </NormalText>
  </section>
);

const RetentionPeriod = () => (
  <section>
    <NormalText>
      We store and retain Personal Data you have provided for as long as
      necessary for your continued use of the Services, pursuant to your
      contract with us, and in compliance with applicable laws and regulations.
    </NormalText>
  </section>
);

const SharingAndDisclosureOfInformation = () => (
  <section>
    <NormalText>
      As a matter of principle, we do not sell, rent, exchange, share or
      otherwise disclose your Personal Data to third parties for marketing
      purposes. If we share or disclose information that we collect, we do so in
      accordance with the practices described in this Privacy Policy. The
      categories of parties with whom and instances where we may share your
      information include, but shall not be limited to:
    </NormalText>
    <List sx={{ listStylePosition: "outside" }}>
      <LatinListItem>
        <Typography variant="body3" fontWeight="bold">
          Affiliates.
        </Typography>{" "}
        We share information with our affiliates and related entities, including
        where they act as our service providers.
      </LatinListItem>
      <LatinListItem>
        <Typography variant="body3" fontWeight="bold">
          Service Providers.
        </Typography>{" "}
        We share information with third-party service providers for business
        purposes, including fraud detection and prevention, security threat
        detection, payment processing, customer support, data analytics,
        information technology, storage, and transaction monitoring. The Company
        shall require its service providers to abide by this Privacy Policy. All
        service providers that we engage with are restricted to only utilizing
        the information on our behalf and in accordance with our instructions.
      </LatinListItem>
      <LatinListItem>
        <Typography variant="body3" fontWeight="bold">
          Professional Advisors.
        </Typography>{" "}
        We share information with our professional advisors for purposes of
        audits and compliance with our legal and regulatory obligations.
      </LatinListItem>
      <LatinListItem>
        <Typography variant="body3" fontWeight="bold">
          Merger or Acquisition.
        </Typography>{" "}
        We share information in connection with, or during negotiations of, any
        proposed or actual merger, purchase, sale or any other type of
        acquisition or business combination of all or any portion of our assets,
        or transfer of all or a portion of our business to another business.
      </LatinListItem>
      <LatinListItem>
        <Typography variant="body3" fontWeight="bold">
          Security and Compelled Disclosure.
        </Typography>{" "}
        We share information to comply with the law or other legal process, and
        where required, in response to lawful requests including to meet
        national security or law enforcement requirements by public authorities,
        law enforcement agencies, data protection authorities or regulatory
        agencies, or government officials.
      </LatinListItem>
      <LatinListItem>
        <Typography variant="body3" fontWeight="bold">
          Facilitating Requests.
        </Typography>{" "}
        We may share information about you at your request or instruction.
        Consent. We may share information about you with your consent.
      </LatinListItem>
      <LatinListItem>
        <Typography variant="body3" fontWeight="bold">
          Other Legitimate Purpose.
        </Typography>{" "}
        We may share your information to pursue the Company’s legitimate
        purposes and for the conclusion or the performance of a contract of for
        the provision of the Services.
      </LatinListItem>
    </List>
    <NormalText>
      Notwithstanding the above, we may share information that does not identify
      you (including information that has been aggregated or de-identified),
      except as otherwise prohibited by applicable law.
    </NormalText>
  </section>
);

const SpecificDisclosuresAndNotices = () => (
  <section>
    <NormalText fontWeight="bold">
      Specific Notice to California Residents (“CCPA Notice”)
    </NormalText>
    <NormalText>
      The California Consumer Privacy Act of 2018 (“CCPA”) requires certain
      businesses to provide a CCPA Notice to explain how a company collects,
      uses, and shares Personal Data of California residents and the rights and
      choices offered regarding the handling of such data or information.
    </NormalText>
    <List sx={{ listStyleType: "disc", listStylePosition: "outside" }}>
      <DiscListItem>
        <Typography variant="body3" fontWeight="bold">
          Privacy Practices.
        </Typography>{" "}
        We will not sell your Personal Data or “personal information” as defined
        under the CCPA.
      </DiscListItem>
      <DiscListItem>
        <Typography variant="body3" fontWeight="bold">
          Privacy Rights.
        </Typography>{" "}
        The CCPA gives individuals the right to request information about how
        the Company has collected, used, and shared your personal information
        and gives you the right to request a copy of any information that we may
        have stored or maintained about you. You may also ask us to delete any
        personal information that we may have received about you. The CCPA
        limits these rights, for example, by prohibiting us from providing
        certain sensitive information in response to access requests and
        limiting the circumstances under which we must comply with a request for
        deletion of personal information. We will respond to requests for
        information, access, and deletion only to the extent that we are able to
        associate, with a reasonable effort, the information we maintain with
        the identifying details you provide in your request. If we deny the
        request, we will communicate this decision to you. You are entitled to
        exercise the rights described above free from discrimination.
      </DiscListItem>
      <DiscListItem>
        <Typography variant="body3" fontWeight="bold">
          Submitting a Request.
        </Typography>{" "}
        You can submit a request for information, access, or deletion to{" "}
        <Link variant="body3" href="mailto:legal@composable.finance">
          legal@composable.finance
        </Link>
        .
      </DiscListItem>
      <DiscListItem>
        <Typography variant="body3" fontWeight="bold">
          Identity Verification.
        </Typography>{" "}
        The CCPA requires us to collect and verify the identity of any
        individual submitting a request to access or delete personal information
        before providing a substantive response.
      </DiscListItem>
      <DiscListItem>
        <Typography variant="body3" fontWeight="bold">
          Authorized Agents.
        </Typography>{" "}
        California residents can designate an “authorized agent” to submit
        requests on their behalf. We will require the authorized agent to have a
        written authorization confirming their authority.
      </DiscListItem>
    </List>
    <NormalText fontWeight="bold">
      Additional Disclosures for European Union Data Subjects or User
    </NormalText>
    <NormalText>
      We will process your Personal Data for the purposes described above, as
      applicable. Our justifications and bases for processing your Personal Data
      include: (1) you have given consent to the process to us or our Service
      provides for one or more specific purposes; (2) processing is necessary
      for the performance of a contract with you; (3) processing is necessary
      for compliance with a legal obligation; and/or (4) processing is necessary
      for any legitimate interests pursued by us or a third party, and your
      interests and fundamental rights and freedoms do not override those
      interests.
    </NormalText>
    <NormalText>
      Your rights under the GDPR include the right to: (1) request access and
      obtain a copy of your Personal Data; (2) request rectification or deletion
      of your personal data; (3) object to or restrict the processing of your
      Personal Data; and (4) request portability of your Personal Data.
      Additionally, you may withdraw your consent to our collection at any time.
      Nevertheless, we cannot edit or delete information that is stored on a
      particular blockchain. Information such as your transaction data,
      blockchain wallet address, and assets held by your address that may be
      related to the data we collect is beyond our control.
    </NormalText>
    <NormalText>
      To exercise any of your rights under the GDPR, please contact us at{" "}
      <Link variant="body3" href="mailto:legal@composable.finance">
        legal@composable.finance
      </Link>
      . We may require additional information from you to process your request.
      Please note that we may retain information as necessary to fulfill the
      purpose for which it was collected and may continue to do so even after a
      data subject request in accordance with our legitimate interests,
      including to comply with our legal obligations, resolve disputes, prevent
      fraud, and enforce our agreements.
    </NormalText>
  </section>
);

const UseOfPersonalData = () => (
  <section>
    <NormalText>
      We may collect and/or use your Personal Data for business purposes in
      accordance with the practices described in this Privacy Policy. Our
      business purposes for collecting, retaining and using information include,
      but are not limited to:
    </NormalText>
    <List sx={{ listStylePosition: "outside" }}>
      <LatinListItem>
        Operating and managing the Services pursuant to a contractual
        obligation; performing Services requested by you, such as responding to
        your comments, questions, and requests, and providing information
        support; sending you technical notices, updates, security alerts,
        information regarding changes to our policies, and support and
        administrative messages; detecting, preventing, and addressing fraud,
        breach of terms, and threats, or harm; and compliance with legal and
        regulatory requirements.
      </LatinListItem>
      <LatinListItem>
        Protecting the security and integrity of the Services and the Company;
        improving the Services and other websites, apps, products and services;
        conducting promotions, such as hack-a-thons, including verification of
        your eligibility and/or delivery of prizes in connection with your
        entries; and fulfilling any other business purpose, with notice to you
        and your consent.
      </LatinListItem>
      <LatinListItem>
        Protecting your privacy rights to prevent any unauthorized use of the
        Services; performing identity verification when you make an inquiry or
        give an instruction to the Company; and sending you documents or
        announcements regarding the Services.
      </LatinListItem>
      <LatinListItem>
        Complying with applicable laws, rules, regulations, codes of practice or
        guidelines issued by any legal or regulatory bodies; responding to
        subpoenas, court orders or similar legal procedures and requests from
        public and government authorities, including public and government
        authorities outside your country of residence.
      </LatinListItem>
    </List>
    <NormalText>
      During the period when you use the Services, your Personal Data may be
      processed and/or used in the form(s) of printed documents and/or
      electronic files or otherwise by the Company and its affiliates
      representatives, agents, advisors and /or service providers within and/or
      outside Saint Lucia.
    </NormalText>
    <NormalText>
      Notwithstanding the above, we may use information that does not identify
      you (including information that has been aggregated or de-identified) for
      any purpose except as otherwise prohibited by applicable law.{" "}
    </NormalText>
  </section>
);

const WithdrawalAndDeactivation = () => (
  <section>
    <NormalText>
      If you decide to stop using our Services, or deactivate your account (if
      applicable), all Personal Data or information collected relating to you
      shall be handled in accordance with Saint Lucia law, this Privacy Policy,
      and the Company’s other policies (if applicable). Such deactivation shall
      not be considered as a withdrawal of consent for the use and disclosure of
      such Personal Data or information by us, unless otherwise expressly
      requested or informed by you in writing.
    </NormalText>
    <NormalText>
      If you decide to withdraw your consent for the Company to use and/or
      disclose all your Personal Data, we will cease to collect your information
      unless there is a legal justification for the continued collection of your
      Personal Data. Further, we may not be able to continue providing our
      Services to you or continue any contractual relationship that is in place
      depending on the nature of your request. The withdrawal of your consent
      may result in the termination of any agreements with us and it may be
      considered as a breach of your contractual obligations or undertakings. In
      such instance, we reserve our legal right to pursue any remedies available
      at law or in equity. The withdrawal of your consent does not in any way
      affect the lawfulness of the collection of your data based on the consent
      given prior to the withdrawal.
    </NormalText>
  </section>
);

const YourRights = () => (
  <section>
    <NormalText>
      Your rights under this Privacy Policy include the right to: (1) request
      access and obtain a copy of your Personal Data; (2) rectify, correct, or
      supplement Personal Data we have collected.; (3) object to or restrict the
      processing of your Personal Data; (4) request portability of your Personal
      Data; (5) object to decisions based solely on automated processing,
      including profiling, unless you have given your consent or the same is
      necessary for the performance of a contract between you and the Company;
      (6) request for the removal or deletion of any of your Personal Data.
      Additionally, if we have collected and processed your Personal Data with
      your consent, you have the right to withdraw your consent at any time. The
      withdrawal of your consent shall, however not affect the lawfulness of
      processing based on your consent before its withdrawal.
    </NormalText>
    <NormalText>
      Notwithstanding the foregoing, we cannot edit or delete information that
      is stored on a particular blockchain. This information may include
      transaction data (i.e., purchases, sales, and transfers) related to your
      blockchain wallet address and any non-fungible tokens (NFTs) held by your
      wallet address.
    </NormalText>
    <NormalText>
      To exercise any of these rights, please contact us via our email or postal
      address provided in this Privacy Policy and specify the right(s) you are
      seeking to exercise. We will respond to your request within thirty (30)
      days. We may require specific information from you to help us confirm your
      identity and process your request. We may retain information, as
      necessary, to fulfill the purpose for which it was collected and may
      continue to retain and use information even after your request in
      accordance with our legitimate interests, including as is it is necessary
      to comply with our legal obligations, resolve disputes, prevent fraud, and
      enforce our agreements. Notwithstanding this, you retain your right to
      lodge a complaint with the data protection regulator in your jurisdiction.
    </NormalText>
    <NormalText>
      While you can choose freely whether or not you will provide the Company
      with your Personal Data, you may be unable to use the Services, or a
      portion thereof, until you have provided the Company with such necessary
      information or data.
    </NormalText>
    <NormalText>
      Your data protection rights are not absolute and we may deny you your
      rights in accordance with the applicable data protections laws by
      providing you with the reason(s) for the denial of your request.
    </NormalText>
  </section>
);

import Default from "@/components/Templates/Default";
import { NextPage } from "next";
import { Typography } from "@mui/material";

const styles = {
  titanium_06: "rgba(255,255,255,0.6)",
  hexplore: "#B265FF",
};

const Link = ({ children }: { children: string }) => {
  return (
    <a
      href={`https://${children}`}
      style={{ color: styles.hexplore, textDecoration: "unset" }}
      target="_blank"
      rel="noopener noreferrer"
    >
      {children}
    </a>
  );
};

const BoldTitle = ({ children }: { children: string }) => {
  return (
    <>
      <br />
      <span style={{fontWeight: 'bold', paddingLeft: '15px'}}>{children}</span>
      <br />
    </>
  );
};

const PrivacyPolicy: NextPage = () => {
  return (
    <Default breadcrumbs={[]}>
      <div style={{ maxWidth: "1032px", margin: "0 auto" }}>
        <div style={{ marginBottom: "48px" }}>
          <Typography variant="h4" fontWeight="bold">
            Privacy Policy
          </Typography>
        </div>
        <Typography
          fontWeight="normal"
          fontSize="14px"
          style={{ cursor: 'default', pointerEvents: 'auto'}}
          color={styles.titanium_06}
          marginBottom={12}
        >
          Privacy Policy
          <br />
          (As of December 13, 2022)
          <br />
          This Privacy Policy, among others, seeks to notify and inform you as a
          user (“you”, “your”, or “user”) of how Composable Finance Ltd.
          (“Composable”, “we”, “our”, “us” or the “Company”) collects, uses,
          manages and shares Personal Data (as hereinafter defined under Saint
          Lucia’s Data Protection Act and under this Privacy Policy) in our
          website at <Link>app.pablo.finance</Link>, and such other websites,
          web apps, or online location that links to this Privacy Policy
          (collectively, the “Site”) and in connection with the services
          provided by the Company through Pablo, a decentralized exchange, or
          through a related affiliate in the Site (collectively, the
          “Services”), as well as your rights and choices regarding such
          Personal Data, in compliance with Saint Lucia’s Data Protection Act,
          the United Kingdom’s (“UK”) Data Protection Act, the European Union’s
          General Data Privacy Regulation (“GDPR”) and other privacy laws,
          rules, and regulations as they may become applicable. This Privacy
          Policy applies to Personal Data collected, used, stored, disclosed
          and/or processed by the Company through the Site and in connection
          with the Services as described in the Terms of Use available at{" "}
          <Link>app.pablo.finance/terms-of-use</Link>.
          <br />
          <br />
          Personal Data, as defined under Saint Lucia’s Data Protection Act,
          refers to information about a data subject that is recorded in any
          form including:
          <br />
          <br />
          <ol type='a' style={{ margin: "0px 0px 0px -15px" }}>
            <li>
              information relating to the race, national or ethnic origin,
              religion, age, sexual orientation, sexual life or marital status
              of the data subject;
            </li>
            <li>
              information relating to the education, medical, criminal or
              employment history of the data subject or information relating to
              the financial transactions in which the individual has been
              involved or which refers to the data subject;
            </li>
            <li>
              any identifying number, symbol or other particular designated to
              the data subject;
            </li>
            <li>
              the address, fingerprints, Deoxyribonucleic Acid (DNA), or blood
              type of the data subject;
            </li>
            <li>
              the name of the data subject where it appears with other personal
              data relating to the data subject or where the disclosure of the
              name itself would reveal information about the data subject;
            </li>
            <li>
              correspondence sent to an establishment by the data subject that
              is explicitly or implicitly of a private or confidential nature,
              and replies to such correspondence that would reveal the contents
              of the original correspondence; or
            </li>
            <li>
              the views or opinions of any other person about the data subject.
            </li>
          </ol>
          <br />
          By submitting information to us, or signing for the Services offered
          by us, you agree and consent to the collection, use, retention and
          disclosure by the Company and any of its affiliates, representatives,
          agents, advisors and/or service providers, as well as any other
          activities described in this Privacy Policy, of your Personal Data and
          other information. If you do not agree with the terms of this Privacy
          Policy, you should immediately discontinue the use of the Services and
          refrain from accessing the Site.
          <br /><br />We reserve the right to revise and
          update this Privacy Policy at any time. Any changes will be effective
          immediately upon our posting of the latest version of the Privacy
          Policy in the Site. Your continued use of the Services indicates your
          consent to the latest version of the Privacy Policy then published or
          posted.
          <br />
          <BoldTitle>1. Collected Information</BoldTitle>
          <br />
          When you interact with our Site and/or avail of our Services, we may collect:
          <br />
          <br />
          <ol type='a' style={{ margin: "0px 0px 0px -15px" }}>
            <li>
              Log Files: This includes, but is not limited to, the files that
              record events that occur in connection with your use of the Site.
              These are created when you view content or otherwise interact with
              the Services.
            </li>
            <li>
              Contact Information. This includes, but is not limited to, your
              name, email address, physical address, nationality or citizenship,
              and country of residence.
            </li>
            <li>
              Financial Information. This includes, but is not limited to, your
              network address, cryptocurrency wallet information and balances,
              transaction history, trading data, trading history, and associated
              fees paid.
            </li>
            <li>
              Transaction Information. This includes, but is not limited to, the
              transactions you make on our Services, such as the type of
              transaction, transaction amount, and timestamp.
            </li>
            <li>
              Correspondence. This includes, but is not limited to, your
              feedback, questionnaire and other survey responses, and
              information you provide to our support teams, including via our
              help chat or social media messaging channels.
            </li>
            <li>
              Online Identifiers. This includes, but is not limited to, your
              username, geographical location or tracking details, browser
              fingerprint, operating system, browser name and version, and
              internet protocol (IP) addresses.
            </li>
            <li>
              Usage and Diagnostics Data. This includes, but is not limited to,
              conversion events, user preferences, crash logs, device
              information and other data collected via cookies and similar
              technologies.
            </li>
            <li>
              Information We Get from Others. This refers to any information
              about you from other sources as required or permitted by
              applicable law, including public databases. We may combine the
              information collected from these sources with the information we
              get from this Site in order to comply with our legal obligations
              and limit the use of our Services in connection with fraudulent or
              other illicit activities.
            </li>
            <li>
              Information from Cookies and other Tracking Technologies. We, and
              third parties we authorize, may use cookies, web beacons, and
              similar technologies to record your preferences, track the use of
              our Sites, including our mobile applications, and collect
              information about the use of the Services, as well as about our
              interactions with you. This information may include IP addresses,
              browser type, internet service provider (ISP), referring/exit
              pages, operating system, device information, date or time stamp,
              and clickstream data, and information about your interactions with
              the communications we send to you. We may combine this
              automatically collected log information with other information we
              collect about you. We will only strictly use necessary cookies in
              connection with the Site and Services. For the avoidance of doubt,
              we use cookies that are essential for you to browse the Site and
              use the Site’s features, including accessing secure areas of the
              Site. You can choose to disable cookies, however, in such
              circumstances you will not be able to use parts of the Services
              which require cookies to be active.
            </li>
          </ol>
          <br />
          In order to improve user experience and for website optimization, and
          to facilitate our internal analysis, we may likewise: (1) store your
          cookie consent state for the current domain; (2) register data or
          information regarding any on-site behavior or actions taken; and (3)
          collect data or information from your navigation and/or interaction in
          the Site.
          <br />
          <BoldTitle>2. Collection of Personal Data</BoldTitle>
          <br />
          Personal Data is collected when you use our Services or when you:
          <br />
          <br />
          <ol type='a' style={{ margin: "0px 0px 0px -15px" }}>
            <li>Deposit cryptocurrency assets</li>
            <li>Make trades</li>
            <li>Withdraw cryptocurrency assets</li>
          </ol>
          <br />
          The is not an exhaustive list of how we collect Personal Data as we
          may also collect Personal Data from other companies or third parties.
          <br />
          <BoldTitle>3. Services and Features</BoldTitle>
          <br />
          The Personal Data we collect is used to provide our Services and the
          Site’s Features as well as maintain and improve our Services as
          described in the Terms of Use. This includes using Personal Data to:
          <br />
          <br />
          <ol type='a' style={{ margin: "0px 0px 0px -15px" }}>
            <li>
              Operate, maintain, customize, measure, and improve our Services;
            </li>
            <li>Create and update user accounts;</li>
            <li>Improve user experience;</li>
            <li>Process transactions;</li>
            <li>
              Send information and marketing communications, including notices,
              updates, security alerts, promotions, surveys, news, events, and
              support and administrative messages;
            </li>
            <li>Created de-identified or aggregated data;</li>
            <li>Maintain safety, security and integrity of the Services;</li>
            <li>Provide customer support;</li>
            <li>
              Test, research, analyze, and develop products or services to
              improve user experience; and
            </li>
            <li>
              Compare the information against third-party databases and public
              records.
            </li>
          </ol>
          <BoldTitle>4. Sharing and Disclosure of Information</BoldTitle>
          <br />
          As a matter of principle, we do not sell, rent, exchange, share or
          otherwise disclose your Personal Data to third parties for marketing
          purposes. If we share or disclose information that we collect, we do
          so in accordance with the practices described in this Privacy Policy.
          The categories of parties with whom and instances where we may share
          your information include, but shall not be limited to:
          <br />
          <br />
          <ol type='a' style={{ margin: "0px 0px 0px -15px" }}>
            <li>
              Affiliates. We share information with our affiliates and related
              entities, including where they act as our service providers.
            </li>
            <li>
              Service Providers. We share information with third-party service
              providers for business purposes, including fraud detection and
              prevention, security threat detection, payment processing,
              customer support, data analytics, information technology, storage,
              and transaction monitoring. The Company shall require its service
              providers to abide by this Privacy Policy. All service providers
              that we engage with are restricted to only utilizing the
              information on our behalf and in accordance with our instructions.
            </li>
            <li>
              Professional Advisors. We share information with our professional
              advisors for purposes of audits and compliance with our legal and
              regulatory obligations.
            </li>
            <li>
              Merger or Acquisition. We share information in connection with, or
              during negotiations of, any proposed or actual merger, purchase,
              sale or any other type of acquisition or business combination of
              all or any portion of our assets, or transfer of all or a portion
              of our business to another business.
            </li>
            <li>
              Security and Compelled Disclosure. We share information to comply
              with the law or other legal process, and where required, in
              response to lawful requests including to meet national security or
              law enforcement requirements by public authorities, law
              enforcement agencies, data protection authorities or regulatory
              agencies, or government officials.
            </li>
            <li>
              Facilitating Requests. We may share information about you at your
              request or instruction.
            </li>
            <li>
              Consent. We may share information about you with your consent.
            </li>
            <li>
              Other Legitimate Purpose. We may share your information to pursue
              the Company’s legitimate purposes and for the conclusion or the
              performance of a contract of for the provision of the Services.
            </li>
          </ol>
          <br />
          Notwithstanding the above, we may share information that does not
          identify you (including information that has been aggregated or
          de-identified), except as otherwise prohibited by applicable law.
          <br />
          <BoldTitle>5. Additional Disclosure</BoldTitle>
          <br />
          This Additional Disclosure governs our collection, use and sharing of
          Personal Data that users provide to us to start or complete the
          trading process. In case of conflicting sections or provisions between
          this Additional Disclosure and other sections or provisions of this
          Privacy Policy, this Additional Disclosure shall govern.
          <br />
          <br />
          When you are no longer using our Services, we continue to share you
          information which includes, but is not limited to the following:
          <br />
          <br />
          <ol type='a' style={{ margin: "0px 0px 0px -15px" }}>
            <li>Contact details</li>
            <li>IP addresses</li>
            <li>Trading history</li>
            <li>Cryptocurrency balances and wallets</li>
            <li>Conversion events</li>
          </ol>
          <br />
          Notwithstanding the above, we may share information, as described in
          this Additional Disclosure, except as otherwise prohibited by
          applicable law.
          <br />
          <BoldTitle>6. Other Parties</BoldTitle>
          <br />
          We may integrate technologies operated or controlled by other parties
          into parts of the Services. For example, the Services may include
          links that hyperlink to websites, platforms, and other services not
          operated or controlled by us.
          <br />
          <br />
          When you interact with other parties, including when you leave the
          Site, the other parties may independently collect information about
          you and solicit information from you. The information collected and
          stored by other parties remains subject to their own policies and
          practices, including the information they share with us, your rights
          and choices on their services and devices, and whether they store
          information anywhere in the world. We do not control such other
          parties or any of their content. We shall not be held liable or be
          responsible for the contents and privacy policies of or services
          provided by the other parties. We encourage you to read and
          familiarize yourself with their privacy policies and terms of use.
          <br />
          <BoldTitle>7. Data Security</BoldTitle>
          <br />
          We implement and maintain reasonable administrative, physical, and
          technical security safeguards to help protect your Personal Data from
          outside attacks or threats, loss, theft, misuse, unauthorized access,
          disclosure, alteration, and destruction. Nevertheless, we do not
          provide any guarantee regarding the effectiveness of these protective
          measures or our ability to prevent other parties, acting unlawfully,
          from gaining access to your Personal Data. You are responsible for all
          activity on the Pablo protocol relating to any of your network
          addresses or cryptocurrency wallets.
          <br />
          <BoldTitle>8. International Transfer</BoldTitle>
          <br />
          Information collected through the Services may be transferred to,
          processed, stored, and used in the European Economic Area (“EEA”), the
          UK, and other jurisdictions. Data protection laws in the EEA, the UK
          and other jurisdictions may be different from the laws of your country
          of residence. Your use of the Services or provision of any
          information, therefore, constitutes your consent to the transfer to
          and from, processing, usage, sharing, and storage of your Personal
          Data in the EEA, the UK and other jurisdictions as set out in this
          Privacy Policy.
          <br />
          <BoldTitle>9. Your Rights</BoldTitle>
          <br />
          Your rights under this Privacy Policy include the right to: (1)
          request access and obtain a copy of your Personal Data; (2) rectify,
          correct, or supplement Personal Data we have collected.; (3) object to
          or restrict the processing of your Personal Data; (4) request
          portability of your Personal Data; (5) object to decisions based
          solely on automated processing, including profiling, unless you have
          given your consent or the same is necessary for the performance of a
          contract between you and the Company; (6) request for the removal or
          deletion of any of your Personal Data. Additionally, if we have
          collected and processed your Personal Data with your consent, you have
          the right to withdraw your consent at any time. The withdrawal of your
          consent shall, however not affect the lawfulness of processing based
          on your consent before its withdrawal.
          <br />
          <br />
          Notwithstanding the foregoing, we cannot edit or delete information
          that is stored on a particular blockchain. This information may
          include transaction data (i.e., purchases, sales, and transfers)
          related to your blockchain wallet address and any non-fungible tokens
          (NFTs) held by your wallet address.
          <br />
          <br />
          To exercise any of these rights, please contact us via our email or
          postal address provided in this Privacy Policy and specify the
          right(s) you are seeking to exercise. We will respond to your request
          within thirty (30) days. We may require specific information from you
          to help us confirm your identity and process your request. We may
          retain information, as necessary, to fulfill the purpose for which it
          was collected and may continue to retain and use information even
          after your request in accordance with our legitimate interests,
          including as is it is necessary to comply with our legal obligations,
          resolve disputes, prevent fraud, and enforce our agreements.
          Notwithstanding this, you retain your right to lodge a complaint with
          the data protection regulator in your jurisdiction.
          <br />
          <br />
          While you can choose freely whether or not you will provide the
          Company with your Personal Data, you may be unable to use the
          Services, or a portion thereof, until you have provided the Company
          with such information or data if it is necessary for your use or
          continued use of the Services.
          <br />
          <br />
          Your data protection rights are not absolute and we may deny you your
          rights in accordance with the applicable data protections laws by
          providing you with the reason(s) for the denial of your request.
          <br />
          <BoldTitle>10. Retention Period</BoldTitle>
          <br />
          We store and retain Personal Data you have provided for as long as
          necessary for your continued use of the Services, pursuant to your
          contract with us, and in compliance with applicable laws and
          regulations.
          <br />
          <BoldTitle>11. Withdrawal and Deactivation</BoldTitle>
          <br />
          If you decide to stop using our Services, or deactivate your account
          (if applicable), all Personal Data or information collected relating
          to you shall be handled in accordance with Saint Lucia law, this
          Privacy Policy, and the Company’s other policies. Such deactivation
          shall not be considered as a withdrawal of consent for the use and
          disclosure of such Personal Data or information by us, unless
          otherwise expressly requested or informed by you in writing.
          <br />
          <br />
          If you decide to withdraw your consent for the Company to use and/or
          disclose all your Personal Data, we will cease to collect your
          information unless there is a legal justification for the continued
          collection of your Personal Data. Further, we may not be able to
          continue providing our Services to you or continue any contractual
          relationship that is in place depending on the nature of your request.
          The withdrawal of your consent may result in the termination of any
          agreements with us and it may be considered as a breach of your
          contractual obligations or undertakings. In such instance, we reserve
          our legal right to pursue any remedies available at law or in equity.
          The withdrawal of your consent does not in any way affect the
          lawfulness of the collection of your data based on the consent given
          prior to the withdrawal.
          <br />
          <BoldTitle>12. Legal Age</BoldTitle>
          <br />
          The Services are intended for general audiences and are directed to
          users who are of legal age. Minors who lack the legal capacity under
          any applicable laws are generally not permitted to access or use our
          Services. Any person who becomes aware of or believes that a minor is
          using our Services should immediately contact us. We shall remove any
          information collected from the minor who may have used our Services
          without our knowledge to ensure compliance with applicable laws.
          <br />
          <BoldTitle>13. Applicable Law and Jurisdiction</BoldTitle>
          <br />
          This Privacy Policy shall be governed by and construed in
          accordance with the laws of Saint Lucia, without regard to
          conflict of law principles. Any disputes arising in respect of
          this Privacy Policy shall be submitted to the exclusive
          jurisdiction of the courts of Saint Lucia.
          <br />
          <BoldTitle>14. Specific Disclosures and Notices</BoldTitle>
          <br />
          <span style={{textDecoration: 'underline', fontWeight: 'bold', paddingLeft: '15px'}}>Specific Notice to California Residents (“CCPA Notice”)</span>
          <br />
          <br />
          The California Consumer Privacy Act of 2018 (“CCPA”) requires
          certain businesses to provide a CCPA Notice to explain how a
          company collects, uses, and shares Personal Data of California
          residents and the rights and choices offered regarding the
          handling of such data or information.
          <br />
          <br />
          <ul style={{ margin: "0px 0px 0px -15px" }}>
            <li>
              Privacy Practices. We will not sell your Personal Data or
              “personal information” as defined under the CCPA.
            </li>
            <li>
              Privacy Rights. The CCPA gives individuals the right to request
              information about how the Company has collected, used, and shared
              your personal information and gives you the right to request a
              copy of any information that we may have stored or maintained
              about you. You may also ask us to delete any personal information
              that we may have received about you. The CCPA limits these rights,
              for example, by prohibiting us from providing certain sensitive
              information in response to access requests and limiting the
              circumstances under which we must comply with a request for
              deletion of personal information. We will respond to requests for
              information, access, and deletion only to the extent that we are
              able to associate, with a reasonable effort, the information we
              maintain with the identifying details you provide in your request.
              If we deny the request, we will communicate this decision to you.
              You are entitled to exercise the rights described above free from
              discrimination.
            </li>
            <li>
              Submitting a Request. You can submit a request for information,
              access, or deletion to{" "}
              <a
                style={{
                  textDecoration: "underline",
                  color: styles.titanium_06,
                }}
                href="mailto:legal@composable.finance"
              >
                legal@composable.finance.
              </a>
            </li>
            <li>
              Identity Verification. The CCPA requires us to collect and verify
              the identity of any individual submitting a request to access or
              delete personal information before providing a substantive
              response.
            </li>
            <li>
              Authorized Agents. California residents can designate an
              “authorized agent” to submit requests on their behalf. We will
              require the authorized agent to have a written authorization
              confirming their authority.
            </li>
          </ul>
          <br />
          <span style={{fontWeight: 'bold', textDecoration: 'underline', paddingLeft: '15px'}}>Additional Disclosures for European Union Data Subjects or User</span>
          <br />
          <br />
          We will process your Personal Data for the purposes described above, as
          applicable. Our justifications and bases for processing your Personal
          Data include: (1) you have given consent to the process to us or our
          Service provides for one or more specific purposes; (2) processing is
          necessary for the performance of a contract with you; (3) processing
          is necessary for compliance with a legal obligation; and/or (4)
          processing is necessary for any legitimate interests pursued by us or
          a third party, and your interests and fundamental rights and freedoms
          do not override those interests.
          <br />
          <br />
          Your rights under the GDPR include the right to: (1) request access
          and obtain a copy of your Personal Data; (2) request rectification or
          deletion of your personal data; (3) object to or restrict the
          processing of your Personal Data; and (4) request portability of your
          Personal Data. Additionally, you may withdraw your consent to our
          collection at any time. Nevertheless, we cannot edit or delete
          information that is stored on a particular blockchain. Information
          such as your transaction data, blockchain wallet address, and assets
          held by your address that may be related to the data we collect is
          beyond our control.
          <br />
          <br />
          To exercise any of your rights under the GDPR, please contact us at
          legal@composable.finance. We may require additional information from
          you to process your request. Please note that we may retain
          information as necessary to fulfill the purpose for which it was
          collected and may continue to do so even after a data subject request
          in accordance with our legitimate interests, including to comply with
          our legal obligations, resolve disputes, prevent fraud, and enforce
          our agreements.
          <br />
          <BoldTitle>15. Contact Us</BoldTitle>
          <br />
          If you have any requests pursuant to the above provisions, questions
          or comments about this Privacy Policy, our data practices, or our
          compliance with applicable law, please contact us by email at:
          <a
            style={{
              textDecoration: "underline",
              color: styles.titanium_06,
            }}
            href="mailto:legal@composable.finance"
          >legal@composable.finance</a> or by mail at: 1st Floor, The Sotheby
          Building, Rodney Village, Rodney Bay, LC 04 101, Gros Islet, Saint
          Lucia.
          <br />
          <br />
        </Typography>
      </div>
    </Default>
  );
};
export default PrivacyPolicy;

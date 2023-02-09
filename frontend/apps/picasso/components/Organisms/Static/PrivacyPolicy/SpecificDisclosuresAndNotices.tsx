import { NormalText } from "@/components/Organisms/Static/NormalText";
import { Typography } from "@mui/material";
import { DiscListItem } from "../DiscListItem";
import { List } from "../List";
import { Link } from "@/components";

export const SpecificDisclosuresAndNotices = () => (
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

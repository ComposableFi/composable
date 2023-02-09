import { NormalText } from "@/components/Organisms/Static/NormalText";
import { List } from "../List";
import { LatinListItem } from "@/components/Organisms/Static/LatinListItem";
import { Typography } from "@mui/material";

export const SharingAndDisclosureOfInformation = () => (
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

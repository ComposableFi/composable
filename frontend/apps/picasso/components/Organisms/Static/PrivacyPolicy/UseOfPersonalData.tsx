import { NormalText } from "@/components/Organisms/Static/NormalText";
import { List } from "../List";
import { LatinListItem } from "@/components/Organisms/Static/LatinListItem";

export const UseOfPersonalData = () => (
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

import { Link } from "@/components/Molecules";
import { NormalText } from "@/components/Organisms/Static/NormalText";

export const Preface = () => (
  <section>
    <NormalText>
      These Terms of Use (“Terms”) constitute a binding and enforceable legal
      contract between Composable Finance Ltd. and its affiliates (“Composable,”
      “we,” “us,” or the “Company”) and you, an end user of the services (“you”
      or “User”) at{" "}
      <Link variant="body3" href="https://picasso.xyz">
        https://picasso.xyz
      </Link>{" "}
      and{" "}
      <Link variant="body3" href="/">
        https://app.picasso.xyz/
      </Link>{" "}
      (the “Services”). These Terms also include any guidelines, announcements,
      additional terms, policies, and disclaimers made available or issued by us
      from time to time. By accessing, using or clicking on our website (and all
      related subdomains) or its mobile applications (the “Site”) or accessing,
      using or attempting to use the Services, you agree that you have read,
      understood, and are bound by these Terms and that you shall comply with
      the requirements listed herein. If you do not agree to any of these Terms
      or comply with the requirements herein, please do not access or use the
      Site or the Services.
    </NormalText>
    <NormalText>
      We reserve the right, in our sole discretion, to make changes or
      modifications to the Site and these Terms at any time and for any reason.
      You will be subject to, and will be deemed to have been made aware of and
      to have accepted, any such changes by your continued use of the Site.
    </NormalText>
  </section>
);

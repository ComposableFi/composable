import { NormalText } from "@/components/Organisms/Static/NormalText";
import { Link } from "@/components";

const subHeading = {
  fontSize: "1rem",
  fontWeight: "bold",
  mb: 0.5,
  mt: 1.125,
};
export const DisputeResolution = () => {
  return (
    <section>
      <NormalText>
        Please read this section carefully: it may significantly affect your
        legal rights, including your right to file a lawsuit in court and to
        have a jury hear your claims. It contains procedures for mandatory
        binding arbitration and a class action waiver.
      </NormalText>
      <NormalText sx={subHeading}>Good Faith Negotiations</NormalText>
      <NormalText>
        Prior to commencing any legal proceeding against us of any kind,
        including an arbitration as set forth below, you and we agree that we
        will attempt to resolve any dispute, claim, or controversy between us
        arising out of or relating to these Terms, the Site, and the Services
        (each, a “Dispute” and, collectively, “Disputes”) by engaging in good
        faith negotiations. For any Dispute you have against Composable, you
        agree to first contact Composable and attempt to resolve the claim
        informally by sending a written notice of your claim (“Notice”) to
        Composable by email at{" "}
        <Link href="mailto:legal@composable.finance" variant="body3">
          legal@composable.finance
        </Link>{" "}
        or by certified mail addressed to Fortgate Offshore Investment and Legal
        Services Ltd., Ground Floor, The Sotheby Building, Rodney Village,
        Rodney Bay, Gros-Islet, Saint Lucia. The Notice must (a) include your
        name, residence address, email address, and telephone number; (b)
        describe the nature and basis of the Dispute; and (c) set forth the
        specific relief sought. Our notice to you will be similar in form to
        that described above. The party receiving such notice shall have thirty
        (30) days to respond to the notice. Within sixty (60) days after the
        aggrieved party sent the initial notice, the parties shall meet and
        confer in good faith by videoconference, or by telephone, to try to
        resolve the dispute. If the parties are unable to resolve the Dispute
        within ninety (90) days after the aggrieved party sent the initial
        notice, the parties may agree to mediate their Dispute, or either party
        may submit the Dispute to arbitration as set forth below.
      </NormalText>
      <NormalText sx={subHeading}>No Representative Actions</NormalText>
      <NormalText>
        You and Composable agree that any Dispute arising out of or related to
        these Terms, including access and use of the Site and Services, are
        personal to you and Composable and that any Dispute will be resolved
        solely through individual action, and will not be brought as a class
        arbitration, class action or any other type of representative
        proceeding.
      </NormalText>
      <NormalText sx={subHeading}>Agreement to Arbitrate</NormalText>
      <NormalText>
        You and we are each waiving the right to a trial by jury and to have any
        Dispute/s resolved in court. You and we agree that any Dispute that
        cannot be resolved through the procedures set forth above will be
        resolved through binding arbitration in accordance with the
        International Arbitration Rules of the International Centre for Dispute
        Resolution. The place of arbitration shall be in St. Lucia. The language
        of the arbitration shall be English. The arbitrator(s) shall have
        experience adjudicating matters involving internet technology, software
        applications, financial transactions and, ideally, blockchain
        technology. The prevailing party will be entitled to an award of their
        reasonable attorney’s fees and costs. Except as may be required by law,
        neither a party nor its representatives may disclose the existence,
        content, or results of any arbitration hereunder without the prior
        written consent of both parties.
      </NormalText>
      <NormalText sx={subHeading}>Opting Out</NormalText>
      <NormalText>
        You have the right to opt out of binding arbitration within fifteen (15)
        days after the expiry of the 90-day period for good faith negotiations
        and the parties are unable to resolve the Dispute by mailing an opt-out
        notice to Composable at Fortgate Offshore Investment and Legal Services
        Ltd., Ground Floor, The Sotheby Building, Rodney Village, Rodney Bay,
        Gros-Islet, Saint Lucia. In order to be effective, the opt-out notice
        must include your full name and address and clearly indicate your intent
        to opt out of binding arbitration. By opting out of binding arbitration,
        you are agreeing to resolve the Dispute in accordance with the
        provisions on governing law and venue provided in these Terms.
      </NormalText>
    </section>
  );
};

import { DefaultLayout, Link, PageTitle } from "@/components";
import { Grid } from "@mui/material";
import { List } from "@/components/Organisms/Static/List";
import { ListItem } from "@/components/Organisms/Static/ListItem";
import { NormalText } from "@/components/Organisms/Static/NormalText";
import { LatinListItem } from "@/components/Organisms/Static/LatinListItem";

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
const AccessAndAcceptance = () => (
  <section>
    <NormalText>
      By accessing or interacting with this Site, our Services, and/or any of
      the Composable protocols, you hereby acknowledge and accept the foregoing
      obligations and conditions outlined in these Terms.
    </NormalText>
  </section>
);

const Assignment = () => (
  <section>
    <NormalText>
      These Terms may be assigned without your prior consent to any of
      Composable’s affiliates and its successors in the interest of any business
      associated with the Services provided by us. You may not assign or
      transfer any rights or obligations under this agreement without our prior
      written consent.
    </NormalText>
  </section>
);

const AssumptionOfRisks = () => (
  <section>
    <NormalText>You represent and warrant that you:</NormalText>
    <List>
      <LatinListItem>
        Have the necessary technical expertise and ability to review and
        evaluate the security, integrity and operation of any transactions that
        you engage in through the Site or Services;
      </LatinListItem>
      <LatinListItem>
        Have the knowledge, experience, understanding, professional advice and
        information to make your own evaluation of the merits, risks and
        applicable compliance requirements under applicable law of engaging in
        transactions through the Site or Services;
      </LatinListItem>
      <LatinListItem>
        Understand and agree to the inherent risks associated with cryptographic
        systems and blockchain-based networks, digital assets, including the
        usage and intricacies of native digital assets, smart contract-based
        tokens (including fungible tokens and NFTs), and systems that interact
        with blockchain-based networks. Composable does not own or control all
        of the underlying software through which other blockchain networks are
        formed. For example, the software underlying certain blockchain networks
        is open source, such that anyone can use, copy, modify, and distribute
        it;
      </LatinListItem>
      <LatinListItem>
        Acknowledge that Composable’s underlying software and software
        application, including the Digital Assets such as the financial NFTs
        (“fNFTs”), are still new, unproven and in the early development stage.
        There is an inherent risk that the software could contain weaknesses,
        vulnerabilities, or bugs causing, inter alia, the complete loss of the
        Digital Assets, or the risk the Digital Assets may not have their
        intended functionalities or may have no functions at all;
      </LatinListItem>
      <LatinListItem>
        Acknowledge that any use or interaction with the Services requires a
        comprehensive understanding of applied cryptography and computer science
        to appreciate the inherent risks, including those listed above. You
        represent and warrant that you possess relevant knowledge and skills.
        Any reference to a type of digital asset on the Site or otherwise during
        the use of the Services does not indicate our approval or disapproval of
        the technology on which the digital asset relies, and should not be used
        as a substitute for your understanding of the risks specific to each
        type of digital asset;
      </LatinListItem>
      <LatinListItem>
        Acknowledge and understand that cryptography is a progressing field with
        advances in code cracking or other technical advancements, such as the
        development of quantum computers, which may present risks to digital
        assets and the Services, and could result in the theft or loss of your
        digital assets. To the extent possible, we intend to update
        Composable-developed smart contracts related to the Services to account
        for any advances in cryptography and to incorporate additional security
        measures necessary to address risks presented from technological
        advancements, but that intention does not guarantee or otherwise ensure
        full security of the Services;
      </LatinListItem>
      <LatinListItem>
        Acknowledge and agree that (a) Composable is not solely responsible for
        the operation of the blockchain-based software and networks underlying
        the Services, (b) there exists no guarantee of the functionality,
        security, or availability of that software and networks, and (c) the
        underlying blockchain-based networks are subject to sudden changes in
        operating rules, such as those commonly referred to as “forks,” which
        may materially affect the Services;
      </LatinListItem>
      <LatinListItem>
        Understand that Pablo and the Picasso parachain remains under
        development, which creates technological and security risks when using
        the Services in addition to uncertainty relating to digital assets and
        transactions therein. You acknowledge that the cost of transacting in
        Pablo and the Picasso parachain is variable and may increase at any time
        causing impact to any activities taking place therein, which may result
        in price fluctuations or increased costs when using the Services;
      </LatinListItem>
      <LatinListItem>
        Understand that blockchain networks use public and private key
        cryptography. Thus, you alone are responsible for securing your private
        key(s). We do not have access to your private key(s). Losing control of
        your private key(s) will permanently and irreversibly deny you access to
        any of your digital assets on the network. Neither Composable nor any
        other person or entity will be able to retrieve or protect your digital
        assets. If your private key(s) are lost, then you will not be able to
        transfer your digital assets to any other blockchain address or wallet.
        If this occurs, then you will not be able to realize any value or
        utility from the digital assets that you may hold;
      </LatinListItem>
      <LatinListItem>
        Understand that the markets for these digital assets are highly volatile
        due to factors including (but not limited to) adoption, speculation,
        technology, security, and regulation;
      </LatinListItem>
      <LatinListItem>
        Confirm that you are solely responsible for your use of the Services,
        including all of your transfers of Digital Assets and all the trades you
        place, including any erroneous orders that may be filled. We do not take
        any action to resolve erroneous trades or transfers that result from
        your mistake or inadvertence;
      </LatinListItem>
      <LatinListItem>
        Acknowledge that Composable’s underlying software and software
        application are still in an early development stage and unproven. There
        is an inherent risk that the software could contain weaknesses,
        vulnerabilities, or bugs causing, inter alia, the complete loss of
        digital assets and tokens.
      </LatinListItem>
      <LatinListItem>
        Acknowledge that the Services are subject to flaws and that you are
        solely responsible for evaluating any code provided by the Services or
        Site. This warning and other warnings that Composable provides in these
        Terms are in no way evidence or represent an on-going duty to alert you
        to all of the potential risks of utilizing the Services or accessing the
        Site;
      </LatinListItem>
      <LatinListItem>
        Understand all the risks relating to supplying Digital Assets to a
        liquidity to a liquidity pool including the risk of impermanent loss
        wherein the value of one Digital Asset rises in comparison to the other
        resulting in the assets being worth less in paper which may become
        permanent if you withdraw liquidity before the price stabilizes;
      </LatinListItem>
      <LatinListItem>
        Acknowledge and accept that these Digital Assets, especially the fNFTs,
        are relatively new innovations and technologies in DeFi whose regulatory
        treatment are presently uncertain, which could lead to them being
        considered as securities and subject to restrictions under applicable
        law concerning their purchase, sale, trade and related transactions. Any
        unintended violation of law may lead to enforcement action and
        penalties. Also, there is a risk with new technologies that they may not
        function entirely as intended or create unknown risks;
      </LatinListItem>
      <LatinListItem>
        Acknowledge and accept the risk that your digital assets may lose some
        or all of their value while they are supplied to the App, you may suffer
        large and immediate financial loss due to the fluctuation of prices of
        Digital Assets in a liquidity pool or trading pair, and may experience
        price slippage and cost. Thus, you should not hold value you cannot
        afford to lose in digital assets;
      </LatinListItem>
      <LatinListItem>
        Agree and accept that the Services and your digital assets could be
        impacted by one or more regulatory inquiries or regulatory actions,
        which could impede or limit the ability of Composable to continue to
        make available our proprietary software and could impede or limit your
        ability to access or use the Services;
      </LatinListItem>
      <LatinListItem>
        Understand that anyone can create a token, including fake versions of
        existing tokens and tokens that falsely claim to represent projects, and
        acknowledge and accept the risk that you may mistakenly trade those or
        other tokens;
      </LatinListItem>
      <LatinListItem>
        Understand that digital assets and tokens may be subject to
        expropriation and/or theft by hackers or other malicious groups by
        obstructing the token smart contract which creates the tokens in a
        variety of ways, including, but not limited to, malware attacks, denial
        of service attacks, consensus-based attacks, Sybil attacks, smurfing and
        spoofing;
      </LatinListItem>
      <LatinListItem>
        Understand and accept that DEXes require no form of Know-Your-Customer
        due diligence before users can trade and anyone with a crypto wallet can
        trade on DEXes without any discrimination, which, thus, may potentially
        increase the risk of interacting with malicious or fraudulent users;
      </LatinListItem>
      <LatinListItem>
        Acknowledge that DEXes, because of their decentralized nature, are not
        presently subject to comprehensive regulation;
      </LatinListItem>
      <LatinListItem>
        Acknowledge and accept that the cost and speed of transacting with
        cryptographic and blockchain-based systems are variable and may increase
        at any time;
      </LatinListItem>
      <LatinListItem>
        Understand and accept that you are solely responsible for reporting and
        paying any taxes applicable to your use of the Services;
      </LatinListItem>
      <LatinListItem>
        Confirm and accept that there are risks associated with the use of the
        Site and Services that Composable cannot anticipate. Such risks may
        appear as unanticipated variations or combinations of the risks
        discussed above;
      </LatinListItem>
      <LatinListItem>
        Understand and accept that Composable has the right to disable or modify
        access to the Site and the Services (such as restricting features of the
        Services) at any time in the event of any breach of these Terms,
        including, if we reasonably believe any of your representations and
        warranties may be untrue or inaccurate, and we will not be liable to you
        for any losses or damages you may suffer as a result of or in connection
        with the Site or the Services being inaccessible to you at any time or
        for any reason;
      </LatinListItem>
      <LatinListItem>
        Understand that the Site and the Services may evolve, which means
        Composable may apply changes, replace, or discontinue (temporarily or
        permanently) the Services at any time in our sole discretion; and
      </LatinListItem>
      <LatinListItem>
        Assume, and agree that Composable will have no responsibility or
        liability for any and all the risks associated with the use of the Site
        and Services, including, but not limited to the above and you hereby
        irrevocably waive, release and discharge all claims, whether known or
        unknown to you, against Composable, its affiliates and their respective
        shareholders, members, directors, officers, employees, agents and
        representatives related to any of the risks set forth herein.
      </LatinListItem>
    </List>
  </section>
);

const subHeading = {
  fontSize: "1rem",
  fontWeight: "bold",
  mb: 0.5,
  mt: 1.125,
};
const DisputeResolution = () => {
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

const Eligibility = () => (
  <section>
    <NormalText>
      The Site is intended for users who are of legal age. All users who are
      minors in the jurisdiction in which they reside (generally under the age
      of 18) must have the permission of, and be directly supervised by, their
      parent or guardian to use the Site. If you are a minor, you must have your
      parent or guardian read and agree to these Terms prior to you using the
      Site.
    </NormalText>
    <NormalText>
      The information provided on the Site is not intended for distribution to
      or use by any person or entity in any jurisdiction or country where such
      distribution or use would be contrary to law or regulation or which would
      subject us to any registration requirement within such jurisdiction or
      country. Accordingly, those persons who choose to access the Site from
      other locations do so on their own initiative and are solely responsible
      for compliance with local laws, if and to the extent local laws are
      applicable.
    </NormalText>
    <NormalText>
      Additionally, you also represent and warrant that you are not a citizen or
      resident of a state, country, territory or other jurisdiction that is
      embargoed by the United States or where your use of the Site or the
      Services would be illegal or otherwise violate any applicable law.
      Specifically, you represent that you are not located in, organized in, or
      a resident of Cuba, Iran, Syria, North Korea, Russia, Crimea, Donetsk,
      Luhansk, Afghanistan, Balkans, Belarus, Burman, Central African Republic,
      Congo, Ethiopia, Hong Kong, Iraq, Libya, Lebanon, Nicaragua, Somalia,
      Sudan and Darfur, South Sudan, Ukraine, Venezuela, Yemen, Zimbabwe or any
      other jurisdiction where the applicable law prohibits you from accessing
      or using the Services; and you represent that you are not named in the
      Office of Foreign Asset Control of the U.S. Department of the Treasury’s
      Specially Designated and Blocked Persons List.
    </NormalText>
    <NormalText>
      By accessing or using the Site, you agree that you are solely and entirely
      responsible for compliance with all laws and regulations that may apply to
      you.
    </NormalText>
  </section>
);

const EntireAgreement = () => (
  <section>
    <NormalText>
      These Terms and any policies or operating rules posted by us on the
      Services constitute the entire agreement and understanding between you and
      us and govern your access and use of the Site and Services, superseding
      any prior or contemporaneous agreements, communications, and proposals,
      whether oral or written, between you and us (including, but not limited
      to, any prior versions of these Terms). Any failure by us to exercise or
      enforce any right or provision of these Terms shall not constitute a
      waiver of such right or provision.
    </NormalText>
  </section>
);

const ForwardLookingStatements = () => (
  <section>
    <NormalText>
      This information contains “forward-looking statements.” These statements,
      identified by words such as “plan,” “anticipate,” “believe,” “estimate,”
      “should,” “expect,” “will,” “can,” and similar future-looking expressions
      include our expectations and objectives regarding our future operating
      results and business strategy. Forward-looking statements involve known
      and unknown risks, uncertainties, assumptions and other factors that may
      cause the actual results, performance or achievements of Composable and
      its affiliated entities or related projects to be materially different
      from any future results, performance or achievements expressed or implied
      by the forward-looking statements. Such factors include, among others,
      general business, economic, competitive, political and social
      uncertainties; dependence on commercial product interest; as well as
      regulatory or legal changes and uncertainty. Forward-looking statements
      are based on a number of material factors and assumptions, economic
      conditions in the near to medium future, the average cost of the Company’s
      offerings compared to traditional offerings, fluctuations or changes to
      the tax and other regulatory requirements regarding DeFi and the industry
      as a whole. While the Company considers these facts and assumptions to be
      reasonably based on information currently available to it, these
      assumptions may prove to be incorrect. Actual results may vary from such
      forward-looking information for a variety of reasons, including but not
      limited to risks and uncertainties known and unknown by the Company.
      Because forward-looking statements relate to the future, they are subject
      to inherent uncertainties, risks and changes in circumstances that are
      difficult to predict and many of which are outside of our control. The
      Company’s actual results and conditions may differ materially from those
      indicated in the forward-looking statements. Therefore, you should not
      rely on any of these forward-looking statements.
    </NormalText>
  </section>
);

const GoverningLawAndVenue = () => (
  <section>
    <NormalText>
      These Terms and any separate agreements whereby we provide you Services
      shall be governed by and construed in accordance with the laws of the
      Saint Lucia. Any dispute between the Parties that is not subject to
      arbitration will be resolved in Saint Lucia.
    </NormalText>
  </section>
);

const Indemnification = () => (
  <section>
    <NormalText>
      You hereby agree to defend, indemnify, and hold Composable harmless from
      and against any loss, damage, liability, claim, or demand, including
      reasonable attorneys’ fees and expenses, made by any third party due to or
      arising out of: (a) your access and use of the Site and Services; (b) your
      breach or alleged breach of these Terms; (c) any breach of your
      representations and warranties set forth in these Terms; (d) anything you
      contribute to the Services (e) your misuse of the Services, or any smart
      contract and/or script related thereto, (f) your violation of the rights
      of a third party, including but not limited to intellectual property
      rights, publicity, confidentiality, property, or privacy rights (g) any
      overt harmful act toward any other user of the Services with whom you
      connected via the Services; or (h) your violation of any laws, rules,
      regulations, codes, statutes, ordinances, or orders of any governmental or
      quasi-governmental authorities. Notwithstanding the foregoing, we reserve
      the right, at your expense, to assume the exclusive defense and control of
      any matter for which you are required to indemnify us, and you agree to
      cooperate, at your expense, with our defense of such claims. You will not
      in any event settle any claim without our prior written consent.
    </NormalText>
  </section>
);

const IndemnificationAndFullRelease = () => (
  <section>
    <NormalText>
      You agree to hold harmless, release, defend, and indemnify Composable and
      its officers, directors, employees, contractors, agents, affiliates, and
      subsidiaries from and against all claims, damages, obligations, losses,
      liabilities, costs, and expenses arising from: (a) your access and use of
      the Site; (b) your violation of any term or condition of these Terms, the
      right of any third party, or any other applicable law, rule, or
      regulation; and (c) any other party&apos;s access and use of the Site with
      your assistance or using any device or account that you own or control.
    </NormalText>
    <NormalText>
      You likewise expressly agree that you assume all risks in connection with
      your access and use or interaction with the Site, our Services, and/or the
      Composable protocols, as applicable. You further expressly waive and
      release us from any and all liability, claims, causes of action, or
      damages arising from or in any way relating to your use of or interaction
      with the Site, our Services, and/or the Composable protocols.
    </NormalText>
  </section>
);

const InformationalResource = () => (
  <section>
    <NormalText>
      All information, including graphs, charts, tokenomics, projections, and
      other data, provided in connection with your access to the Site and the
      Services are for general informational purposes only and subject to change
      at the sole discretion of Composable. Composable provides resources about
      the fundamentals of the Composable protocol or system, which seeks to
      build a trustless infrastructure for DeFi. This information is not
      intended to be comprehensive or address all aspects of the protocol. You
      should not take, or refrain from taking, any action based on any
      information contained on the Site or any other information that we make
      available at any time, including blog posts, data, articles, links to
      third-party content, news feeds, tutorials, tweets, and videos.
    </NormalText>
    <NormalText>
      The materials appearing in the Site could include technical,
      typographical, or photographic errors. Composable does not warrant that
      any of the materials on its website are accurate, complete or current.
    </NormalText>
  </section>
);

const IntellectualPropertyRights = () => (
  <section>
    <NormalText>
      Unless otherwise indicated, the Site is our proprietary property and all
      source code, databases, functionality, software, website designs, audio,
      video, text, photographs, and graphics on the Site (collectively, the
      “Content”) and the trademarks, service marks, and logos contained therein
      (the “Marks”) are owned or controlled by us or licensed to us, and are
      protected by copyright and trademark laws and various other intellectual
      property rights and unfair competition laws of the applicable
      jurisdiction, international copyright laws, and international conventions.
      The Content and the Marks are provided on the Site “AS IS” for your
      information and personal use only. Except as expressly provided in these
      Terms, no part of the Site and no Content or Marks may be copied,
      reproduced, aggregated, republished, uploaded, posted, publicly displayed,
      encoded, translated, transmitted, distributed, sold, licensed, or
      otherwise exploited for any commercial purpose whatsoever, without our
      express prior written permission.
    </NormalText>
  </section>
);

const LimitationOfLiability = () => (
  <section>
    <NormalText>
      Under no circumstances shall Composable or any of its officers, directors,
      employees, contractors, agents, affiliates, or subsidiaries be liable for
      any indirect, punitive, incidental, special, consequential, or exemplary
      damages, including (but not limited to) damages for loss of profits,
      goodwill, use, data, or other intangible property, arising out of or
      relating to any access or use of the Site, nor will Composable be
      responsible for any damage, loss, or injury resulting from hacking,
      tampering, or other unauthorized access or use of the Site or the
      information contained within it. Composable assumes no liability or
      responsibility for any: (a) errors, mistakes, or inaccuracies of content;
      (b) personal injury or property damage, of any nature whatsoever,
      resulting from any access or use of the Site; (c) unauthorized access or
      use of any secure server or database in our control, or the use of any
      information or data stored therein; (d) interruption or cessation of
      function related to the Site; (e) bugs, viruses, trojan horses, or the
      like that may be transmitted to or through the Site; (f) errors or
      omissions in, or loss or damage incurred as a result of the use of, any
      content made available through the Site; and (g) the defamatory,
      offensive, or illegal conduct of any third party. Under no circumstances
      shall Composable or any of its officers, directors, employees,
      contractors, agents, affiliates, or subsidiaries be liable to you for any
      claims, proceedings, liabilities, obligations, damages, losses, or costs
      in an amount exceeding the amount you paid to us in exchange for access to
      and use of the Site, or USD$50.00, whichever is greater. This limitation
      of liability applies regardless of whether the alleged liability is based
      on contract, tort, negligence, strict liability, or any other basis, and
      even if we have been advised of the possibility of such liability. Some
      jurisdictions do not allow the exclusion of certain warranties or the
      limitation or exclusion of certain liabilities and damages. Accordingly,
      some of the disclaimers and limitations set forth in these Terms may not
      apply to you. This limitation of liability shall apply to the fullest
      extent permitted by law.
    </NormalText>
  </section>
);

const NotAnOffering = () => (
  <section>
    <NormalText>
      Any information in this Site does not constitute an offer to sell or a
      solicitation of an offer to purchase securities, assets, including digital
      assets, or financial instruments by the Company, or to enter into a
      transaction involving any such security or financial instrument. Such an
      offer can only be done through a registered or licensed offering or
      subject to an exemption. The recipient should not rely upon anything
      within this information in making a decision to participate in the
      Company’s issuances or to utilize the Company’s technology. The Company is
      not required to update the information provided and the information is
      only current as of the date of its release and is subject to change over
      time.
    </NormalText>
    <NormalText>
      Any information provided in this Site is not intended for distribution to
      or use by any person or entity in any jurisdiction or country where such
      distribution or use would be contrary to law or regulation or which would
      subject us to any registration requirement within such jurisdiction or
      country. Accordingly, those persons who choose to access the Site from
      other locations do so on their own initiative and are solely responsible
      for compliance with local laws, if and to the extent local laws are
      applicable.
    </NormalText>
  </section>
);

const NotProfessionalAdvice = () => (
  <section>
    <NormalText>
      All information provided by the Site or Services is for informational
      purposes only and should not be construed as professional advice. You
      should not take, or refrain from taking, any action based on any
      information contained in the Site or Services. Before you make any
      financial, legal, or other decisions involving the Site or Services, you
      should seek independent professional advice from an individual who is
      licensed and qualified in the area, subject matter and jurisdiction for
      which such advice would be appropriate.
    </NormalText>
    <NormalText>
      Composable is not your broker, lawyer, intermediary, agent, or advisor and
      has no fiduciary relationship or obligation to you regarding any decisions
      or activities that you have undertaken or will be undertaking when using
      the Site or the Services. Neither our communications nor any information
      that we provide to you is intended as, or shall be considered or construed
      as advice.
    </NormalText>
  </section>
);

const NoWarranties = () => (
  <section>
    <NormalText>
      THE SITE IS PROVIDED ON AN AS-IS AND AS-AVAILABLE BASIS. YOU AGREE THAT
      YOUR USE OF THE SITE AND OUR SERVICES WILL BE AT YOUR SOLE RISK. TO THE
      FULLEST EXTENT PERMITTED BY LAW, WE DISCLAIM ALL WARRANTIES, EXPRESS OR
      IMPLIED, IN CONNECTION WITH THE SITE AND YOUR USE THEREOF, INCLUDING,
      WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF MERCHANTABILITY, FITNESS FOR
      A PARTICULAR PURPOSE, AND NON-INFRINGEMENT. WE MAKE NO WARRANTIES OR
      REPRESENTATIONS ABOUT THE ACCURACY OR COMPLETENESS OF THE SITE’S CONTENT,
      MATERIALS, AND INFORMATION OR THE CONTENT OF ANY WEBSITES LINKED TO THE
      SITE AND WE WILL ASSUME NO LIABILITY OR RESPONSIBILITY FOR ANY (1) ERRORS,
      MISTAKES, OR INACCURACIES OF CONTENT, INFORMATION, AND/OR MATERIALS, (2)
      PERSONAL INJURY OR PROPERTY DAMAGE, OF ANY NATURE WHATSOEVER, RESULTING
      FROM YOUR ACCESS TO AND USE OF THE SITE, (3) ANY UNAUTHORIZED ACCESS TO OR
      USE OF OUR SECURE SERVERS AND/OR ANY AND ALL PERSONAL INFORMATION AND/OR
      FINANCIAL INFORMATION STORED THEREIN, (4) ANY INTERRUPTION OR CESSATION OF
      TRANSMISSION TO OR FROM THE SITE, (5) ANY BUGS, VIRUSES, TROJAN HORSES, OR
      THE LIKE WHICH MAY BE TRANSMITTED TO OR THROUGH THE SITE BY ANY THIRD
      PARTY, AND/OR (6) ANY ERRORS OR OMISSIONS IN ANY CONTENT AND MATERIALS OR
      FOR ANY LOSS OR DAMAGE OF ANY KIND INCURRED AS A RESULT OF THE USE OF ANY
      CONTENT POSTED, TRANSMITTED, OR OTHERWISE MADE AVAILABLE VIA THE SITE. WE
      DO NOT WARRANT, ENDORSE, GUARANTEE, OR ASSUME RESPONSIBILITY FOR ANY
      PRODUCT OR SERVICE ADVERTISED OR OFFERED BY A THIRD PARTY THROUGH THE
      SITE, ANY HYPERLINKED WEBSITE, OR ANY WEBSITE OR MOBILE APPLICATION
      FEATURED IN ANY BANNER OR OTHER ADVERTISING, AND WE WILL NOT BE A PARTY TO
      OR IN ANY WAY BE RESPONSIBLE FOR MONITORING ANY TRANSACTION BETWEEN YOU
      AND ANY THIRD-PARTY PROVIDERS OF PRODUCTS OR SERVICES. AS WITH THE
      PURCHASE OF A PRODUCT OR SERVICE THROUGH ANY MEDIUM OR IN ANY ENVIRONMENT,
      YOU SHOULD USE YOUR BEST JUDGMENT AND EXERCISE CAUTION WHERE APPROPRIATE.
    </NormalText>
  </section>
);

const Preface = () => (
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

const ReservedRights = () => (
  <section>
    <NormalText>
      Composable reserves the following rights: (a) with or without notice to
      you, to modify, substitute, eliminate or add to the Site; (b) to review,
      modify, filter, disable, delete and remove any and all content and
      information from the Site; and (c) to cooperate with any law enforcement,
      court or government investigation or order or third party requesting or
      directing that we disclose information or content or information that you
      provide.
    </NormalText>
  </section>
);

const Staking = () => (
  <section>
    <NormalText>
      The website application available at{" "}
      <Link href="/staking" variant="body3">
        https://app.picasso.xyz/staking
      </Link>{" "}
      (the “App”) may allow “staking” wherein the User voluntarily locks in
      Digital Assets into a protocol in exchange for rewards or incentives which
      can be in the form of other types of Digital Assets (the “Staking
      Rewards”). The Staking Rewards, for example, may come in the form of a
      transferable non-fungible token (NFT) which represents the User’s staked
      Digital Assets and all additional rewards and incentives relating thereto.
      It is important to note, however, that the continued existence, form, or
      annual percentage rate (APR) of these Staking Rewards are in no way
      guaranteed and are subject to changes or modification from time to time
      and even complete withdrawal.
    </NormalText>
  </section>
);

const ThirdPartyWebsiteAndContent = () => (
  <section>
    <NormalText>
      The Site may contain (or you may be sent via the Site) links to other
      websites (“Third-Party Websites”) as well as articles, photographs, text,
      graphics, pictures, designs, music, sound, video, information,
      applications, software, and other content or items belonging to or
      originating from third parties (“Third-Party Content”). Such Third-Party
      Websites and Third-Party Content are not investigated, monitored, or
      checked for accuracy, appropriateness, or completeness by us, and we are
      not responsible for any Third-Party Websites accessed through the Site or
      any Third-Party Content posted on, available through, or installed from
      the Site, including the content, accuracy, offensiveness, opinions,
      reliability, privacy practices, or other policies of or contained in the
      Third-Party Websites or the Third-Party Content. Inclusion of, linking to,
      or permitting the use or installation of any Third-Party Websites or any
      Third-Party Content does not imply approval or endorsement thereof by us.
      If you decide to leave the Site and access the Third-Party Websites or to
      use or install any Third-Party Content, you do so at your own risk, and
      you should be aware that these Terms no longer govern. You agree and
      acknowledge that we do not endorse the products or services offered on
      Third-Party Websites and you shall hold us harmless from any losses or
      injury caused by your purchase of such products or services. Additionally,
      you shall hold us harmless from any losses sustained by you or harm caused
      to you relating to or resulting in any way from any Third-Party Content or
      any contact with Third-Party Websites.
    </NormalText>
  </section>
);

const UnacceptableUseOrConduct = () => (
  <section>
    <NormalText>
      As a condition to accessing or using the Site or the Services, you will
      not:
    </NormalText>
    <List
      sx={{
        listStylePosition: "outside",
      }}
    >
      <LatinListItem>
        Violate any applicable law, including, without limitation, any relevant
        and applicable anti-money laundering and anti-terrorist financing laws,
        such as the Bank Secrecy Act, each as may be amended;
      </LatinListItem>
      <LatinListItem>
        Infringe on or misappropriate any contract, intellectual property or
        other third-party right, or commit a tort while using the Site or the
        Services;
      </LatinListItem>
      <LatinListItem>
        Use the Site or Services in any manner that could interfere with,
        disrupt, negatively affect, or inhibit other users from fully enjoying
        the Site or Services, or that could damage, disable, overburden, or
        impair the functioning of the Site or Services in any manner;
      </LatinListItem>
      <LatinListItem>
        Attempt to circumvent any content filtering techniques or security
        measures that Composable employs on the Site, or attempt to access any
        service or area of the Site or the Services that you are not authorized
        to access;
      </LatinListItem>
      <LatinListItem>
        Use the Services to pay for, support, or otherwise engage in any illegal
        gambling activities, fraud, money-laundering, or terrorist activities,
        or other illegal activities;
      </LatinListItem>
      <LatinListItem>
        Use any robot, spider, crawler, scraper, or other automated means or
        interface not provided by us, to access the Services or to extract data;
      </LatinListItem>
      <LatinListItem>
        Introduce any malware, virus, Trojan horse, worm, logic bomb, drop-dead
        device, backdoor, shutdown mechanism or other harmful material into the
        Site or the Services;
      </LatinListItem>
      <LatinListItem>
        Provide false, inaccurate, or misleading information;
      </LatinListItem>
      <LatinListItem>
        Post content or communications on the Site that are, in our sole
        discretion, libelous, defamatory, profane, obscene, pornographic,
        sexually explicit, indecent, lewd, vulgar, suggestive, harassing,
        hateful, threatening, offensive, discriminatory, bigoted, abusive,
        inflammatory, fraudulent, deceptive or otherwise objectionable;
      </LatinListItem>
      <LatinListItem>
        Post content on the Site containing unsolicited promotions, political
        campaigning, or commercial messages or any chain messages or user
        content designed to deceive or trick the user of the Services; or
      </LatinListItem>
      <LatinListItem>
        Use the Site or the Services from a jurisdiction that we have, in our
        sole discretion, or a relevant governmental authority has determined is
        a jurisdiction where the use of the Site or the Services is prohibited.
      </LatinListItem>
    </List>
  </section>
);

const UseCases = () => (
  <section>
    <NormalText>
      Use cases are provided for illustration purposes only and the Company
      neither supports nor condones any such uses. Any such uses should be done
      at the risk of the person making the utilization of the technology and any
      user agrees to hold harmless and indemnify the Company from and against
      any losses that may occur as a result of any such uses.
    </NormalText>
  </section>
);

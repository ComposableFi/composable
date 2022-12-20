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

const TermsOfUse: NextPage = () => {
  return (
    <Default breadcrumbs={[<></>]}>
      <div style={{ maxWidth: "1032px", margin: "0 auto" }}>
        <div style={{ marginBottom: "48px" }}>
          <Typography variant="h4" fontWeight="bold">
            Terms of use
          </Typography>
        </div>
        <Typography
          fontWeight="normal"
          fontSize="14px"
          color={styles.titanium_06}
          marginBottom={12}
        >
          Terms of Use
          <br />
          As of December 13, 2022
          <br />
          <br />
          These Terms of Use (“Terms”) constitute a binding and enforceable
          legal contract between Composable Finance Ltd. and all its affiliates
          (“Composable,” “we,” “us,” or the “Company”) and you, an end user of
          the services (“you” or "User”) of Pablo or its features described
          further below and available at app.pablo.xyz (the “Services”). These
          Terms include guidelines, announcements, additional terms, policies,
          and disclaimers made available or issued by us from time to time.
          <br />
          <br />
          Pablo is a new-generation, cross-chain decentralized exchange (“DEX”)
          built into the Picasso parachain and combines the concept of
          protocol-owned liquidity within the design of a DEX and aims to
          provide a superior trading experience and deep liquidity. The protocol
          consists of free, publicly available, open-source software, including
          smart contracts that are deployed on the Kusama and Polkadot
          blockchains.
          <br />
          <br />
          By accessing, using, or clicking on, website (and all related
          subdomains), webapp, or mobile applications (if any) of Pablo (the
          “Site”) or accessing, using or attempting to use the Services thereon,
          you agree that you have read, understood, and are bound by these Terms
          and that you shall comply with the requirements listed herein. If you
          do not agree to any of these Terms or comply with the requirements
          herein, please do not access or use the Site or the Services.
          <br />
          <br />
          These Terms govern your use of the Site to access the order book,
          matching engine, smart contracts, decentralized applications, APIs and
          all other software that Composable, Pablo or a third party has
          developed for trading or receiving cryptocurrencies, coins or tokens
          and, without limitation, other blockchain-based assets such as
          non-fungible tokens or “NFTs” (collectively, “Digital Assets”) and
          exchanging one Digital Asset for another Digital Asset. These Terms
          expressly cover your rights and obligations, and our disclaimers and
          limitations of legal liability, relating to your use of, and access
          to, the Site and the Services.
          <br />
          <br />
          We reserve the right, in our sole discretion, to make changes or
          modifications to the Site and these Terms at any time and for any
          reason. You will be subject to, and will be deemed to have been made
          aware of and to have accepted, any such changes by your continued use
          of the Site.
          <br />
          <br />
          1. Eligibility
          <br />
          <br />
          The Site is intended for users who are at least eighteen (18) years of
          age. All users who are minors in the jurisdiction in which they reside
          (generally under the age of 18) must have the permission of, and be
          directly supervised by, their parent or guardian to use the Site. If
          you are a minor, you must have your parent or guardian read and agree
          to these Terms prior to you using the Site.
          <br />
          <br />
          The information provided on the Site is not intended for distribution
          to or use by any person or entity in any jurisdiction or country where
          such distribution or use would be contrary to law or regulation or
          which would subject us to any registration requirement within such
          jurisdiction or country. Accordingly, those persons who choose to
          access the Site from other locations do so on their own initiative and
          are solely responsible for compliance with local laws, if and to the
          extent local laws are applicable.
          <br />
          <br />
          Additionally, you also represent and warrant that you are not a
          citizen or resident of a state, country, territory or other
          jurisdiction that is embargoed by the United States or where your use
          of the Site or the Services would be illegal or otherwise violate any
          applicable law. Specifically, you represent that you are not located
          in, organized in, or a resident of Cuba, Iran, Syria, North Korea,
          Russia, Crimea, Donetsk, Luhansk, or any other jurisdiction where the
          applicable law prohibits you from accessing or using the Services
          (“Restricted Territory”); and you represent that you are not named in
          the Office of Foreign Asset Control of the U.S. Department of the
          Treasury’s Specially Designated and Blocked Persons List.
          <br />
          <br />
          The Services are not offered to persons or entities who reside in, are
          citizens of, are located in, are incorporated in, or have a registered
          office in the United States of America (collectively, “US Persons”).
          Moreover, no Services are offered to persons or entities who reside
          in, are citizens of, are located in, are incorporated in, or have a
          registered office in any Restricted Territory. We do not make
          exceptions; therefore, if you are a US Person, then do not attempt to
          use our perpetual contracts and if you are a restricted person, then
          do not attempt to use any of the services. Use of a virtual private
          network (VPN) to circumvent the restrictions set forth herein is
          prohibited.
          <br />
          <br />
          1. Informational Resource
          <br />
          <br />
          All information, including graphs, charts, tokenomics, projections,
          and all other data, provided in connection with your access to the
          Site and the Services are for general informational purposes only and
          subject to change at the sole discretion of Composable. Composable
          provides resources about the fundamentals of the Pablo protocol. This
          information is not intended to be comprehensive or address all aspects
          of the protocol. You should not take, or refrain from taking, any
          action based on any information contained on the Site or any other
          information that we make available at any time, including blog posts,
          data, articles, links to third-party content, news feeds, tutorials,
          tweets, and videos.
          <br />
          <br />
          The materials appearing in the Site could include technical,
          typographical, or photographic errors or inaccuracies. Composable does
          not warrant that any of the materials on Pablo are accurate, complete
          or current and does not undertake to periodically review the materials
          for accuracy, completeness or recency.
          <br />
          <br />
          1. Intellectual Property Rights
          <br />
          <br />
          Unless otherwise indicated, the Site is our proprietary property and
          all source code, databases, smart contracts, functionality, software,
          website designs, audio, video, text, photographs, and graphics on the
          Site (collectively, the “Content”) and the trademarks, service marks,
          and logos contained therein (the “Marks”) are owned or controlled by
          us or licensed to us, and are protected by copyright and trademark
          laws and various other intellectual property rights and unfair
          competition laws of the applicable jurisdiction, international
          copyright laws, and international conventions. The Content and the
          Marks are provided on the Site “AS IS” for your information and
          personal use only. Except as expressly provided in these Terms, no
          part of the Site and no Content or Marks may be copied, reproduced,
          aggregated, republished, uploaded, posted, publicly displayed,
          encoded, translated, transmitted, distributed, sold, licensed, or
          otherwise exploited for any commercial purpose whatsoever, without our
          express prior written permission.
          <br />
          <br />
          1. Third-Party Website and Content
          <br />
          <br />
          The Site may contain (or you may be sent via the Site) links to other
          websites (“Third-Party Websites”) as well as articles, photographs,
          text, graphics, pictures, designs, music, sound, video, information,
          applications, software, and other content or items belonging to or
          originating from third parties (“Third-Party Content”). Such
          Third-Party Websites and Third-Party Content are not investigated,
          monitored, or checked for accuracy, appropriateness, or completeness
          by us, and we are not responsible for any Third-Party Websites
          accessed through the Site or any Third-Party Content posted on,
          available through, or installed from the Site, including the content,
          accuracy, offensiveness, opinions, reliability, privacy practices, or
          other policies of or contained in the Third-Party Websites or the
          Third-Party Content. Inclusion of, linking to, or permitting the use
          or installation of any Third-Party Websites or any Third-Party Content
          does not imply approval or endorsement thereof by us. If you decide to
          leave the Site and access the Third-Party Websites or to use or
          install any Third-Party Content, you do so at your own risk, and you
          should be aware that these Terms no longer govern. You agree and
          acknowledge that we do not endorse the products or services offered on
          Third-Party Websites and you shall hold us harmless from any losses or
          injury caused by your purchase of such products or services.
          Additionally, you shall hold us harmless from any losses sustained by
          you or harm caused to you relating to or resulting in any way from any
          Third-Party Content or any contact with Third-Party Websites.
          <br />
          <br />
          1. Unacceptable Use or Conduct
          <br />
          <br />
          As a condition to accessing or using the Site or the Services, you
          will not:
          <br />
          <br />
          <ol style={{ margin: "0px 0px 0px -15px" }}>
            <li>
              Violate any applicable law, including, without limitation, any
              relevant and applicable anti-money laundering and anti-terrorist
              financing laws (such as the Bank Secrecy Act of the United
              States), as may be amended;{" "}
            </li>
            <li>
              Infringe on or misappropriate any contract, intellectual property
              or other third-party right, or commit a tort while using the Site
              or the Services;{" "}
            </li>
            <li>
              Use the Site or Services in any manner that could interfere with,
              disrupt, negatively affect, or inhibit other users from fully
              enjoying the Site or Services, or that could damage, disable,
              overburden, or impair the functioning of the Site or Services in
              any manner;{" "}
            </li>
            <li>
              Attempt to circumvent any content filtering techniques or security
              measures that Composable employs on the Site, or attempt to access
              any service or area of the Site or the Services that you are not
              authorized to access;{" "}
            </li>
            <li>
              Use the Services to pay for, support, or otherwise engage in any
              illegal gambling activities, fraud, money-laundering, or terrorist
              activities, or other illegal activities;{" "}
            </li>
            <li>
              Transfer or transact with Digital Assets that you have no legal
              right in or was not lawfully obtained by you;{" "}
            </li>
            <li>
              Use any robot, spider, crawler, scraper, or other automated means
              or interface not provided by us, to access the Services or to
              extract data, or introduce any malware, virus, Trojan horse, worm,
              logic bomb, drop-dead device, backdoor, shutdown mechanism or
              other harmful material into the Site or the Services;{" "}
            </li>
            <li>
              Engage in improper or abusive trading practices, including (a) any
              fraudulent act or scheme to defraud, deceive, trick or mislead;
              (b) trading ahead of another user of the Services or
              front-running; (c) fraudulent trading; (d) accommodation trading;
              (e) fictitious transactions; (f) pre-arranged or non-competitive
              transactions; (g) violations of bids or offers; (h) spoofing; (i)
              manipulation; (j) spoofing; (k) knowingly making any bid or offer
              for the purpose of making a market price that does not reflect the
              true state of the market; or (l) entering orders for the purpose
              of entering into transactions without a net change in either
              party’s open positions but a resulting profit to one party and a
              loss to the other party, commonly known as a “money pass.”{" "}
            </li>
            <li>
              Use or access the Site or the Services to transmit or exchange
              Digital Assets that are the direct or indirect proceeds of any
              criminal or fraudulent activity, including terrorism or tax
              evasion;{" "}
            </li>
            <li>Provide false, inaccurate, or misleading information; </li>
            <li>
              Post content or communications on the Site that are, in our sole
              discretion, libelous, defamatory, profane, obscene, pornographic,
              sexually explicit, indecent, lewd, vulgar, suggestive, harassing,
              hateful, threatening, offensive, discriminatory, bigoted, abusive,
              inflammatory, fraudulent, deceptive or otherwise objectionable;
            </li>
            <li>
              Post content on the Site containing unsolicited promotions,
              political campaigning, or commercial messages or any chain
              messages or user content designed to deceive or trick the user of
              the Services;{" "}
            </li>
            <li>
              Use the Site or the Services from a jurisdiction that we have, in
              our sole discretion, or a relevant governmental authority has
              determined is a jurisdiction where the use of the Site or the
              Services is prohibited;{" "}
            </li>
            <li>
              Harass, abuse or harm another person, including Composable’s
              employees and service providers;{" "}
            </li>
            <li>
              Impersonate another user of the Services or otherwise misrepresent
              yourself;{" "}
            </li>
            <li>
              or Engage or attempt to engage, or encourage, induce or assist any
              third party to engage or attempt to engage in any of the
              activities prohibited under this section any other provision of
              these Terms.
            </li>
          </ol>
          <br />
          <br />
          1. Proprietary Rights
          <br />
          <br />
          You acknowledge that certain aspects of the Site or the Services may
          use, incorporate or link to certain open-source components and that
          your use of the Site or Services is subject to, and you will comply
          with, any applicable open-source licenses that govern any such
          open-source components (collectively, the “Open-Source Licenses”).
          Without limiting the generality of the foregoing, you may not (a)
          resell, lease, lend, share, distribute, or otherwise permit any third
          party to use the Site or the Services; (b) use the Site or the
          Services for time-sharing or service bureau purposes; or (c) otherwise
          use the Site or the Services in a manner that violates the Open-Source
          Licenses.
          <br />
          <br />
          Excluding third-party software that the Site or the Services
          incorporates, as between you and Composable, Composable owns the Site
          and the Services, including all technology, content and other
          materials used, displayed or provided on the Site or in connection
          with the Services (including all intellectual property rights
          subsisting therein, whether or not subject to the Open-Source
          Licenses), and hereby grants you a limited, non-exclusive, revocable,
          non-transferable, non-sublicensable license to access and use those
          portions of the Site and the Services that are proprietary to
          Composable and not available pursuant to the Open-Source Licenses.
          <br />
          <br />
          The Services are non-custodial. When you deposit Digital Assets into
          any Composable-developed smart contract, you retain control over those
          Digital Assets at all times. The private key associated with your
          blockchain address from which you transfer Digital Assets is the only
          private key that can control the Digital Assets you transfer into the
          Composable-developed smart contracts. In some cases, you may withdraw
          digital assets from any Composable-developed smart contract only to
          the blockchain address from which you deposited the Digital Assets.
          <br />
          <br />
          1. Forward-Looking Statements
          <br />
          <br />
          This information contains “forward-looking statements.” These
          statements, identified by words such as “plan,” “anticipate,”
          “believe,” “estimate,” “should,” “expect,” “will,” “can,” and similar
          future-looking expressions include our expectations and objectives
          regarding our future operating results and business strategy.
          Forward-looking statements involve known and unknown risks,
          uncertainties, assumptions and other factors that may cause the actual
          results, performance or achievements of Composable and its affiliated
          entities or related projects to be materially different from any
          future results, performance or achievements expressed or implied by
          the forward-looking statements. Such factors include, among others,
          general business, economic, competitive, political and social
          uncertainties; dependence on commercial product interest; as well as
          regulatory or legal changes and uncertainty. Forward-looking
          statements are based on a number of material factors and assumptions,
          economic conditions in the near to medium future, the average cost of
          the Company’s offerings compared to traditional offerings,
          fluctuations or changes to the tax and other regulatory requirements
          regarding decentralized finance (DeFi) and the industry as a whole.
          While the Company considers these facts and assumptions to be
          reasonably based on information currently available to it, these
          assumptions may prove to be incorrect. Actual results may vary from
          such forward-looking information for a variety of reasons, including
          but not limited to risks and uncertainties known and unknown by the
          Company. Because forward-looking statements relate to the future, they
          are subject to inherent uncertainties, risks and changes in
          circumstances that are difficult to predict and many of which are
          outside of our control. The Company’s actual results and conditions
          may differ materially from those indicated in the forward-looking
          statements. Therefore, you should not rely on any of these
          forward-looking statements.
          <br />
          <br />
          1. Not an Offering
          <br />
          <br />
          Any information in this Site does not constitute an offer to sell or a
          solicitation of an offer to purchase securities, assets, including
          Digital Assets, or financial instruments by the Company, or to enter
          into a transaction involving any such security or financial
          instrument. Such an offer can only be done through a registered or
          licensed offering or subject to an exemption. The recipient should not
          rely upon anything within this information in making a decision to
          participate in the Company’s issuances or to utilize the Company’s
          technology. The Company is not required to update the information
          provided and the information is only current as of the date of its
          release and is subject to change over time.
          <br />
          <br />
          1. Not Professional Advice
          <br />
          <br />
          All information provided by the Site or Services is for informational
          purposes only and should not be construed as professional advice. You
          should not take, or refrain from taking, any action based on any
          information contained in the Site or Services. Before you make any
          financial, legal, or other decisions involving the Site or Services,
          you should seek independent professional advice from an individual who
          is licensed and qualified in the area, subject matter and jurisdiction
          for which such advice would be appropriate.
          <br />
          <br />
          Composable is not your broker, lawyer, intermediary, agent, or advisor
          and has no fiduciary relationship or obligation to you regarding any
          decisions or activities that you have undertaken or will be
          undertaking when using the Site or the Services. Neither our
          communications nor any information that we provide to you is intended
          as, or shall be considered or construed as advice.
          <br />
          <br />
          1. Assumption of Risks
          <br />
          <br />
          You represent and warrant that you:
          <br />
          <br />
          <ol style={{ margin: "0px 0px 0px -15px" }}>
            <li>
              Have the necessary technical expertise and ability to review and
              evaluate the security, integrity and operation of any transactions
              that you engage in through the Site or Services;{" "}
            </li>
            <li>
              Have the knowledge, experience, understanding, professional advice
              and information to make your own evaluation of the merits, risks
              and applicable compliance requirements under applicable law of
              engaging in transactions through the Site or Services;{" "}
            </li>
            <li>
              Understand and agree to the inherent risks associated with
              cryptographic systems and blockchain-based networks, digital
              assets, including the usage and intricacies of native digital
              assets, smart contract-based tokens (including fungible tokens and
              NFTs), and systems that interact with blockchain-based networks.
              Composable does not own or control all of the underlying software
              through which other blockchain networks are formed. For example,
              the software underlying blockchain networks, such as the Ethereum
              blockchain, is open source, such that anyone can use, copy,
              modify, and distribute it;{" "}
            </li>
            <li>
              Acknowledge that any use or interaction with the Services requires
              a comprehensive understanding of applied cryptography and computer
              science to appreciate the inherent risks, including those listed
              above. You represent and warrant that you possess relevant
              knowledge and skills. Any reference to a type of digital asset on
              the Site or otherwise during the use of the Services does not
              indicate our approval or disapproval of the technology on which
              the digital asset relies, and should not be used as a substitute
              for your understanding of the risks specific to each type of
              digital asset;{" "}
            </li>
            <li>
              Acknowledge and understand that cryptography is a progressing
              field with advances in code cracking or other technical
              advancements, such as the development of quantum computers, which
              may present risks to digital assets and the Services, and could
              result in the theft or loss of your digital assets. To the extent
              possible, we intend to update Composable-developed smart contracts
              related to the Services to account for any advances in
              cryptography and to incorporate additional security measures
              necessary to address risks presented from technological
              advancements, but that intention does not guarantee or otherwise
              ensure full security of the Services;{" "}
            </li>
            <li>
              Acknowledge and agree that (a) Composable is not solely
              responsible for the operation of the blockchain-based software and
              networks underlying the Services, (b) there exists no guarantee of
              the functionality, security, or availability of that software and
              networks, and (c) the underlying blockchain-based networks are
              subject to sudden changes in operating rules, such as those
              commonly referred to as “forks,” which may materially affect the
              Services;{" "}
            </li>
            <li>
              Understand that Pablo and the Picasso parachain remains under
              development, which creates technological and security risks when
              using the Services in addition to uncertainty relating to digital
              assets and transactions therein. You acknowledge that the cost of
              transacting in Pablo and the Picasso parachain is variable and may
              increase at any time causing impact to any activities taking place
              therein, which may result in price fluctuations or increased costs
              when using the Services;{" "}
            </li>
            <li>
              Understand that blockchain networks use public and private key
              cryptography. Thus, you alone are responsible for securing your
              private key(s). We do not have access to your private key(s).
              Losing control of your private key(s) will permanently and
              irreversibly deny you access to any of your digital assets on the
              network. Neither Composable nor any other person or entity will be
              able to retrieve or protect your digital assets. If your private
              key(s) are lost, then you will not be able to transfer your
              digital assets to any other blockchain address or wallet. If this
              occurs, then you will not be able to realize any value or utility
              from the digital assets that you may hold;{" "}
            </li>
            <li>
              Understand that the markets for these digital assets are highly
              volatile due to factors including (but not limited to) adoption,
              speculation, technology, security, and regulation;{" "}
            </li>
            <li>
              Confirm that you are solely responsible for your use of the
              Services, including all of your transfers of Digital Assets and
              all the trades you place, including any erroneous orders that may
              be filled. We do not take any action to resolve erroneous trades
              or transfers that result from your mistake or inadvertence;{" "}
            </li>
            <li>
              Acknowledge that Composable’s underlying software and software
              application are still in an early development stage and unproven.
              There is an inherent risk that the software could contain
              weaknesses, vulnerabilities, or bugs causing, inter alia, the
              complete loss of digital assets and tokens.{" "}
            </li>
            <li>
              Acknowledge that the Services are subject to flaws and that you
              are solely responsible for evaluating any code provided by the
              Services or Site. This warning and other warnings that Composable
              provides in these Terms are in no way evidence or represent an
              on-going duty to alert you to all of the potential risks of
              utilizing the Services or accessing the Site;{" "}
            </li>
            <li>
              Acknowledge and accept the risk that your digital assets may lose
              some or all of their value while they are supplied to the
              Services, you may suffer large and immediate financial loss due to
              the fluctuation of prices of tokens in a trading pair or liquidity
              pool, and may experience price slippage and cost. Thus, you should
              not hold value you cannot afford to lose in digital assets;{" "}
            </li>
            <li>
              Agree and accept that the Services and your digital assets could
              be impacted by one or more regulatory inquiries or regulatory
              actions, which could impede or limit the ability of Composable to
              continue to make available our proprietary software and could
              impede or limit your ability to access or use the Services;{" "}
            </li>
            <li>
              Understand that anyone can create a token, including fake versions
              of existing tokens and tokens that falsely claim to represent
              projects, and acknowledge and accept the risk that you may
              mistakenly trade those or other tokens;{" "}
            </li>
            <li>
              Understand that digital assets and tokens may be subject to
              expropriation and/or theft by hackers or other malicious groups by
              obstructing the token smart contract which creates the tokens in a
              variety of ways, including, but not limited to, malware attacks,
              denial of service attacks, consensus-based attacks, Sybil attacks,
              smurfing and spoofing
            </li>
            <li>
              Understand and accept that DEXes require no form of
              Know-Your-Customer due diligence before users can trade and anyone
              with a crypto wallet can trade on DEXes without any
              discrimination, which, thus, may potentially increase the risk of
              interacting with malicious or fraudulent users;{" "}
            </li>
            <li>
              Acknowledge that DEXes, because of their decentralized nature, are
              not presently subject to comprehensive regulation;
            </li>
            <li>
              Acknowledge and accept that the cost and speed of transacting with
              cryptographic and blockchain-based systems are variable and may
              increase at any time;{" "}
            </li>
            <li>
              Understand and accept that you are solely responsible for
              reporting and paying any taxes applicable to your use of the
              Services;{" "}
            </li>
            <li>
              Confirm and accept that there are risks associated with the use of
              the Site and Services that Composable cannot anticipate. Such
              risks may appear as unanticipated variations or combinations of
              the risks discussed above;{" "}
            </li>
            <li>
              Understand and accept that Composable has the right to disable or
              modify access to the Site and the Services (such as restricting
              features of the Services) at any time in the event of any breach
              of these Terms, including, if we reasonably believe any of your
              representations and warranties may be untrue or inaccurate, and we
              will not be liable to you for any losses or damages you may suffer
              as a result of or in connection with the Site or the Services
              being inaccessible to you at any time or for any reason;
            </li>
            <li>
              Understand that the Site and the Services may evolve, which means
              Composable may apply changes, replace, or discontinue (temporarily
              or permanently) the Services at any time in our sole discretion;{" "}
            </li>
            <li>
              and Assume, and agree that Composable will have no responsibility
              or liability for any and all the risks associated with the use of
              the Site and Services, including, but not limited to the above and
              you hereby irrevocably waive, release and discharge all claims,
              whether known or unknown to you, against Composable, its
              affiliates and their respective shareholders, members, directors,
              officers, employees, agents and representatives related to any of
              the risks set forth herein.
            </li>
          </ol>
          <br />
          <br />
          1. Disclosures and Disclaimers
          <br />
          <br />
          Composable is a developer of software. Composable does not operate a
          digital asset or derivatives exchange platform or offer trade
          execution or clearing services and has no oversight, involvement, or
          control concerning your transactions using the Services. All
          transactions between users of Composable-developed software are
          executed peer-to-peer directly between the users’ blockchain addresses
          through a smart contract. You understand that Composable is not
          registered or licensed by any regulatory agency or authority. No such
          agency or authority has reviewed or approved the use of Pablo.
          <br />
          <br />
          THE SITE IS PROVIDED ON AN AS-IS AND AS-AVAILABLE BASIS. YOU AGREE
          THAT YOUR USE OF THE SITE AND OUR SERVICES WILL BE AT YOUR SOLE RISK.
          TO THE FULLEST EXTENT PERMITTED BY LAW, WE DISCLAIM ALL WARRANTIES,
          EXPRESS OR IMPLIED, IN CONNECTION WITH THE SITE AND YOUR USE THEREOF,
          INCLUDING, WITHOUT LIMITATION, THE IMPLIED WARRANTIES OF
          MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, AND
          NON-INFRINGEMENT. WE MAKE NO WARRANTIES OR REPRESENTATIONS ABOUT THE
          ACCURACY OR COMPLETENESS OF THE SITE’S CONTENT, MATERIALS, AND
          INFORMATION OR THE CONTENT OF ANY WEBSITES LINKED TO THE SITE AND WE
          WILL ASSUME NO LIABILITY OR RESPONSIBILITY FOR ANY (1) ERRORS,
          MISTAKES, OR INACCURACIES OF CONTENT, INFORMATION, AND/OR MATERIALS,
          (2) PERSONAL INJURY OR PROPERTY DAMAGE, OF ANY NATURE WHATSOEVER,
          RESULTING FROM YOUR ACCESS TO AND USE OF THE SITE, (3) ANY
          UNAUTHORIZED ACCESS TO OR USE OF OUR SECURE SERVERS AND/OR ANY AND ALL
          PERSONAL INFORMATION AND/OR FINANCIAL INFORMATION STORED THEREIN, (4)
          ANY INTERRUPTION OR CESSATION OF TRANSMISSION TO OR FROM THE SITE, (5)
          ANY BUGS, VIRUSES, TROJAN HORSES, OR THE LIKE WHICH MAY BE TRANSMITTED
          TO OR THROUGH THE SITE BY ANY THIRD PARTY, AND/OR (6) ANY ERRORS OR
          OMISSIONS IN ANY CONTENT AND MATERIALS OR FOR ANY LOSS OR DAMAGE OF
          ANY KIND INCURRED AS A RESULT OF THE USE OF ANY CONTENT POSTED,
          TRANSMITTED, OR OTHERWISE MADE AVAILABLE VIA THE SITE. WE DO NOT
          WARRANT, ENDORSE, GUARANTEE, OR ASSUME RESPONSIBILITY FOR ANY PRODUCT
          OR SERVICE ADVERTISED OR OFFERED BY A THIRD PARTY THROUGH THE SITE,
          ANY HYPERLINKED WEBSITE, OR ANY WEBSITE OR MOBILE APPLICATION FEATURED
          IN ANY BANNER OR OTHER ADVERTISING, AND WE WILL NOT BE A PARTY TO OR
          IN ANY WAY BE RESPONSIBLE FOR MONITORING ANY TRANSACTION BETWEEN YOU
          AND ANY THIRD-PARTY PROVIDERS OF PRODUCTS OR SERVICES. AS WITH THE
          PURCHASE OF A PRODUCT OR SERVICE THROUGH ANY MEDIUM OR IN ANY
          ENVIRONMENT, YOU SHOULD USE YOUR BEST JUDGMENT AND EXERCISE CAUTION
          WHERE APPROPRIATE.
          <br />
          <br />
          1. Limitation of Liability
          <br />
          <br />
          Under no circumstances shall Composable or any of its officers,
          directors, employees, contractors, agents, affiliates, or subsidiaries
          be liable for any indirect, punitive, incidental, special,
          consequential, or exemplary damages, including (but not limited to)
          damages for loss of profits, goodwill, use, data, or other intangible
          property, arising out of or relating to any access or use of the Site,
          nor will Composable be responsible for any damage, loss, or injury
          resulting from hacking, tampering, or other unauthorized access or use
          of the Site or the information contained within it. Composable assumes
          no liability or responsibility for any: (a) errors, mistakes, or
          inaccuracies of content; (b) personal injury or property damage, of
          any nature whatsoever, resulting from any access or use of the Site;
          (c) unauthorized access or use of any secure server or database in our
          control, or the use of any information or data stored therein; (d)
          interruption or cessation of function related to the Site; (e) bugs,
          viruses, trojan horses, or the like that may be transmitted to or
          through the Site; (f) errors or omissions in, or loss or damage
          incurred as a result of the use of, any content made available through
          the Site; and (g) the defamatory, offensive, or illegal conduct of any
          third party. Under no circumstances shall Composable or any of its
          officers, directors, employees, contractors, agents, affiliates, or
          subsidiaries be liable to you for any claims, proceedings,
          liabilities, obligations, damages, losses, or costs in an amount
          exceeding the amount you paid to us in exchange for access to and use
          of the Site, or USD$50.00, whichever is greater. This limitation of
          liability applies regardless of whether the alleged liability is based
          on contract, tort, negligence, strict liability, or any other basis,
          and even if we have been advised of the possibility of such liability.
          Some jurisdictions do not allow the exclusion of certain warranties or
          the limitation or exclusion of certain liabilities and damages.
          Accordingly, some of the disclaimers and limitations set forth in
          these Terms may not apply to you. This limitation of liability shall
          apply to the fullest extent permitted by law.
          <br />
          <br />
          1. Indemnification and Full Release
          <br />
          <br />
          You hereby agree to defend, indemnify, and hold Composable harmless
          from and against any loss, damage, liability, claim, or demand,
          including reasonable attorneys’ fees and expenses, made by any third
          party due to or arising out of: (a) your access and use of the Site
          and Services and any other party's access and use of the Site with
          your assistance or using any device or account that you own or
          control; (b) your breach or alleged breach of these Terms; (c) any
          breach of your representations and warranties set forth in these
          Terms; (d) anything you contribute to the Services (e) your misuse of
          the Services, or any smart contract and/or script related thereto, (f)
          your violation of the rights of a third party, including but not
          limited to intellectual property rights, publicity, confidentiality,
          property, or privacy rights (g) any overt harmful act toward any other
          user of the Services with whom you connected via the Services; or (h)
          your violation of any laws, rules, regulations, codes, statutes,
          ordinances, or orders of any governmental or quasi-governmental
          authorities. Notwithstanding the foregoing, we reserve the right, at
          your expense, to assume the exclusive defense and control of any
          matter for which you are required to indemnify us, and you agree to
          cooperate, at your expense, with our defense of such claims. You will
          not in any event settle any claim without our prior written consent.
          <br />
          <br />
          You likewise expressly agree that you assume all risks in connection
          with your access and use or interaction with the Site, our Services,
          Pablo, and/or other Composable protocols, as applicable. You further
          expressly waive and release us from any and all liability, claims,
          causes of action, or damages arising from or in any way relating to
          your use of or interaction with Pablo, the Site, our Services, and/or
          the Composable protocols.
          <br />
          <br />
          Additionally, you hereby agree that Composable will have no
          responsibility or liability for the risks set forth in the section on
          “Assumption of Risks.” You further irrevocably waive, release and
          discharge all claims, whether known or unknown to you, against
          Composable and our shareholders, members, directors, officers,
          employees, agents, representatives, affiliates, related persons,
          suppliers, and contractors in relation to any of the risks set forth
          in the section on “Assumption of Risks.”
          <br />
          <br />
          1. Dispute Resolution
          <br />
          <br />
          Please read this section carefully: it may significantly affect your
          legal rights, including your right to file a lawsuit in court and to
          have a jury hear your claims. It contains procedures for mandatory
          binding arbitration and a class action waiver.
          <br />
          <br />
          Good Faith Negotiations
          <br />
          <br />
          Prior to commencing any legal proceeding against us of any kind,
          including an arbitration as set forth below, you and we agree that we
          will attempt to resolve any dispute, claim, or controversy between us
          arising out of or relating to these Terms, the Site, and the Services
          (each, a “Dispute” and, collectively, “Disputes”) by engaging in good
          faith negotiations. For any Dispute you have against Composable, you
          agree to first contact Composable and attempt to resolve the claim
          informally by sending a written notice of your claim (“Notice”) to
          Composable by email at legal@composable.finance or by certified mail
          addressed to Fortgate Offshore Investment and Legal Services Ltd.,
          Ground Floor, The Sotheby Building, Rodney Village, Rodney Bay,
          Gros-Islet, Saint Lucia. The Notice must (a) include your name,
          residence address, email address, and telephone number; (b) describe
          the nature and basis of the Dispute; and (c) set forth the specific
          relief sought. Our notice to you will be similar in form to that
          described above. The party receiving such notice shall have thirty
          (30) days to respond to the notice. Within sixty (60) days after the
          aggrieved party sent the initial notice, the parties shall meet and
          confer in good faith by videoconference, or by telephone, to try to
          resolve the dispute. If the parties are unable to resolve the Dispute
          within ninety (90) days after the aggrieved party sent the initial
          notice, the parties may agree to mediate their Dispute, or either
          party may submit the Dispute to arbitration as set forth below.
          <br />
          <br />
          No Representative Actions
          <br />
          <br />
          You and Composable agree that any Dispute arising out of or related to
          these Terms, including access and use of the Site and Services, are
          personal to you and Composable and that any Dispute will be resolved
          solely through individual action, and will not be brought as a class
          arbitration, class action or any other type of representative
          proceeding.
          <br />
          <br />
          Agreement to Arbitrate
          <br />
          <br />
          You and we are each waiving the right to a trial by jury and to have
          any Dispute/s resolved in court. You and we agree that any Dispute
          that cannot be resolved through the procedures set forth above will be
          resolved through binding arbitration in accordance with the
          International Arbitration Rules of the International Centre for
          Dispute Resolution. The place of arbitration shall be in St. Lucia.
          The language of the arbitration shall be English. The arbitrator(s)
          shall have experience adjudicating matters involving internet
          technology, software applications, financial transactions and,
          ideally, blockchain technology. The prevailing party will be entitled
          to an award of their reasonable attorney’s fees and costs. Except as
          may be required by law, neither a party nor its representatives may
          disclose the existence, content, or results of any arbitration
          hereunder without the prior written consent of both parties.
          <br />
          <br />
          Opting Out
          <br />
          <br />
          You have the right to opt out of binding arbitration within fifteen
          (15) days after the expiry of the 90-day period for good faith
          negotiations and the parties are unable to resolve the Dispute by
          mailing an opt-out notice to Composable at Fortgate Offshore
          Investment and Legal Services Ltd., Ground Floor, The Sotheby
          Building, Rodney Village, Rodney Bay, Gros-Islet, Saint Lucia. In
          order to be effective, the opt-out notice must include your full name
          and address and clearly indicate your intent to opt out of binding
          arbitration. By opting out of binding arbitration, you are agreeing to
          resolve the Dispute in accordance with the provisions on governing law
          and venue provided in these Terms.
          <br />
          1. Reserved Rights
          <br />
          <br />
          Composable reserves the following rights: (a) with or without notice
          to you, to modify, substitute, eliminate or add to the Site; (b) to
          review, modify, filter, disable, delete and remove any and all content
          and information from the Site; and (c) to cooperate with any law
          enforcement, court or government investigation or order or third party
          requesting or directing that we disclose information or content or
          information that you provide.
          <br />
          <br />
          1. Assignment
          <br />
          <br />
          These Terms may be assigned without your prior consent to any person
          or entity, including Composable’s affiliates, successors and assigns.
          You may not assign or transfer any rights or obligations under this
          agreement without our prior written consent.
          <br />
          <br />
          1. Governing Law and Venue
          <br />
          <br />
          These Terms and any separate agreements whereby we provide you
          Services shall be governed by and construed in accordance with the
          laws of Saint Lucia. Any dispute between the Parties that is not
          subject to arbitration will be resolved in Saint Lucia.
          <br />
          <br />
          1. Entire Agreement
          <br />
          <br />
          These Terms, including the Privacy Policy and other policies or
          operating rules published by us, constitute the entire agreement and
          understanding between you and us and govern your access and use of
          Pablo, the Site, and Services and/or other Composable protocols,
          superseding any prior or contemporaneous agreements, communications,
          and proposals, whether oral or written, between you and us (including,
          but not limited to, any prior versions of these Terms). Any failure by
          us to exercise or enforce any right or provision of these Terms shall
          not constitute a waiver of such right or provision.
          <br />
          <br />
          1. Access and Acceptance
          <br />
          <br />
          By accessing or interacting with Pablo, the Site, our Services and/or
          any of the other Composable protocols or by acknowledging these Terms
          by other means, you hereby acknowledge and accept the foregoing
          obligations and conditions outlined in these Terms, If you do not
          agree to these Terms, then you must not access or use Pablo, the Site,
          our Services, and/or any of the other Composable protocols.
        </Typography>
      </div>
    </Default>
  );
};
export default TermsOfUse;

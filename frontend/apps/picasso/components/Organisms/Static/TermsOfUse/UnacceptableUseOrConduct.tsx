import { List } from "../List";
import { NormalText } from "../NormalText";
import { LatinListItem } from "@/components/Organisms/Static/LatinListItem";

export const UnacceptableUseOrConduct = () => (
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

import { NormalText } from "../NormalText";
import { Link } from "@/components";

export const Staking = () => (
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

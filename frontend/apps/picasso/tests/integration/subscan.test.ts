import { subscanAccountLink } from "@/defi/polkadot/Networks";
import BigNumber from "bignumber.js";

describe("Subscan Link", () => {
  it("Provides correct link", () => {
    // liviu account
    const actualLink = "https://picasso.subscan.io/account/5w53mgBc2w2kNQZgFBaYT5h79cQQNfv8vUuoa85zUe5VxBvQ";
    const link = subscanAccountLink("picasso", "5w53mgBc2w2kNQZgFBaYT5h79cQQNfv8vUuoa85zUe5VxBvQ");

    expect(link).toEqual(actualLink);
  });
});

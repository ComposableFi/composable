import BigNumber from "bignumber.js";

export function concatU8a (a: Uint8Array, b: Uint8Array): Uint8Array {
    const c = new Uint8Array(a.length + b.length);
    c.set(a);
    c.set(b, a.length);
    return c;
  }

export const stringToBigNumber = (value: string): BigNumber =>
  new BigNumber(value.replaceAll(",", ""));
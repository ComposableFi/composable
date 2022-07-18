import BigNumber from "bignumber.js";

export const fromHexString = (hexString: any) =>
  new Uint8Array(
    hexString.match(/.{1,2}/g).map((byte: any) => parseInt(byte, 16))
  );

export const toHexString = (bytes: any) =>
  Array.prototype.map
    .call(bytes, (x) => ("0" + (x & 0xff).toString(16)).slice(-2))
    .join("");

export function unwrapNumberOrHex(v: string | number) {
  return new BigNumber(v, v.toString().startsWith("0x") ? 16 : 10);
}

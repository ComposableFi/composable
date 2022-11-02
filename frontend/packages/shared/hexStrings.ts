import BigNumber from "bignumber.js";

export const fromHexString = (hexString: any) =>
  new Uint8Array(
    hexString.match(/.{1,2}/g).map((byte: any) => parseInt(byte, 16))
  );

export const toHexString = (bytes: any) =>
  Array.prototype.map
    .call(bytes, (x) => ("0" + (x & 0xff).toString(16)).slice(-2))
    .join("");

export function unwrapNumberOrHex(v: string | number): BigNumber {
  return new BigNumber(v, v.toString().startsWith("0x") ? 16 : 10);
}

/**
 * Returns ascii representation of a hex string
 * @param hexString
 */
export function hexToAscii(hexString: string): string {
  let str = "";
  for (let n = 0; n < hexString.length; n += 2) {
    const v = parseInt(hexString.substr(n, 2), 16);
    if (v) str += String.fromCharCode(v);
  }

  return str;
}

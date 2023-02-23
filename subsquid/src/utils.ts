import * as ss58 from "@subsquid/ss58";
import BigNumber from "bignumber.js";
import { RequestInfo, RequestInit } from "node-fetch";
import { SubstrateExtrinsicSignature } from "@subsquid/substrate-processor";
import { decodeAddress } from "@polkadot/util-crypto";

export function encodeAccount(account: Uint8Array): string {
  return ss58.codec("picasso").encode(account);
}

export function getAccountFromSignature(signature: SubstrateExtrinsicSignature | undefined): string {
  const signatureValue = signature?.address?.value || signature?.address;
  try {
    if (typeof signatureValue === "string") {
      return encodeAccount(decodeAddress(signatureValue));
    }
    return "";
  } catch {
    return "";
  }
}

// Get amount without decimals
export function getAmountWithoutDecimals(amount: bigint, decimals: number): BigNumber {
  return BigNumber(amount.toString()).div(BigNumber(10 ** decimals));
}

export function divideBigInts(a: bigint, b: bigint): number {
  const quote = BigNumber(a.toString());
  const base = BigNumber(b.toString());
  return quote.div(base).toNumber();
}

export const fetch = <TResponse>(url: RequestInfo, init?: RequestInit): Promise<TResponse> =>
  import("node-fetch").then(({ default: nodeFetch }) => {
    return nodeFetch(url, init)
      .then(res => {
        if (res.ok) {
          return res.json();
        }
        throw new Error(res.statusText);
      })
      .then(data => data as TResponse)
      .catch(err => {
        throw new Error(err);
      });
  });

export const fetchRetry = async <TResponse>(url: RequestInfo, init?: RequestInit, retries = 5): Promise<TResponse> => {
  return fetch<TResponse>(url, init)
    .then(res => res)
    .catch(async err => {
      if (retries > 0) {
        await new Promise(resolve => setTimeout(resolve, 1000));
        console.log("Retrying...", url);
        return fetchRetry(url, init, retries - 1);
      }
      throw new Error(err);
    });
};

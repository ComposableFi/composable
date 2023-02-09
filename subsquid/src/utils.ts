import { EventHandlerContext, SubstrateBlock, SubstrateEvent } from "@subsquid/substrate-processor";
import { instance, mock } from "ts-mockito";
import * as ss58 from "@subsquid/ss58";
import { Store } from "@subsquid/typeorm-store";
import { randomUUID } from "crypto";
import BigNumber from "bignumber.js";
import { RequestInfo, RequestInit } from "node-fetch";
import { isInstance } from "class-validator";

const fetch = (url: RequestInfo, init?: RequestInit) =>
  import("node-fetch").then(({ default: fetch }) => fetch(url, init));

export const BOB = "5woQTSqveJemxVbj4eodiBTSVfC4AAJ8CQS7SoyoyHWW7MA6";
export const CHARLIE = "5wr4XcyxyJYQb71PbSPxhqujKnsS9UAydBhSypGvFgh2QXBa";

export function createCtx(storeMock: Store, blockHeight: number): EventHandlerContext<Store, { event: true }> {
  const blockMock: SubstrateBlock = mock<SubstrateBlock>();
  blockMock.height = blockHeight;
  blockMock.timestamp = 123; // TODO: use better example
  const event: SubstrateEvent = mock<SubstrateEvent>();
  event.id = randomUUID();
  const ctxMock: EventHandlerContext<Store, { event: true }> = mock<EventHandlerContext<Store, { event: true }>>();
  const ctx: EventHandlerContext<Store, { event: true }> = instance(ctxMock);
  ctx.store = instance(storeMock);
  ctx.block = blockMock;
  ctx.event = event;
  if (ctx.event.extrinsic?.signature?.address) {
    ctx.event.extrinsic.signature.address = BOB;
  }

  return ctx;
}

export function createAccount(): Uint8Array {
  return ss58.codec("picasso").decode(BOB);
}

export function encodeAccount(account: Uint8Array): string {
  return ss58.codec("picasso").encode(account);
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

export async function getCoingeckoPrice(assetId: "4" | "130", date?: Date): Promise<number> {
  let time = new Date();
  if (date && isInstance(date, Date)) {
    time = date;
  } else if (date) {
    time = new Date(date);
  }

  const month = time.getMonth() + 1;
  const day = time.getDate();
  const year = time.getFullYear();

  const queryDate = `${day < 10 ? "0" : ""}${day}-${month < 10 ? "0" : ""}${month}-${year}`;

  let coinId: string;
  switch (assetId) {
    case "130": {
      coinId = "tether";
      break;
    }
    case "4": {
      coinId = "kusama";
      break;
    }
    default: {
      throw new Error("Invalid assetId");
    }
  }

  const endpoint = `https://api.coingecko.com/api/v3/coins/${coinId}/history?date=${queryDate}&localization=en`;
  const res = await fetch(endpoint);
  if (!res.ok) {
    throw new Error("Failed to fetch price from coingecko");
  }
  const json: { market_data: { current_price: { usd: number } } } = await res.json();
  const price = json?.market_data?.current_price?.usd;

  return price;
}

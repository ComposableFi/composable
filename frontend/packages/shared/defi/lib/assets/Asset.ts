import { ApiPromise } from "@polkadot/api";
import { fromChainIdUnit } from "../../unit";
import { DropdownOptionWithIcon } from "../types";
import BigNumber from "bignumber.js";
import { pipe } from "fp-ts/function";
import { option } from "fp-ts";
import { TokenId } from "tokens";
import { AssetRatio } from "./picasso";
import { SubstrateNetworkId } from "../../../SubstrateNetworks";

export class Asset {
  protected __api?: ApiPromise;
  protected readonly __name: string;
  protected readonly __symbol: string;
  protected readonly __iconUrl: string;
  protected readonly __tokenId: TokenId;
  protected __price: BigNumber = new BigNumber(0);
  protected __decimals: Map<string, number>;
  protected __ratio: AssetRatio | null;
  protected __existentialDeposit: Map<SubstrateNetworkId, BigNumber | null>;
  protected __parachainAssetIds: Map<string, BigNumber>;

  /**
   * Transform assets list
   * to dropdown options
   * @param {Array<Asset>} assets
   * @returns {DropdownOptionWithIcon[]}
   */
  static toDropdownList(assets: Asset[]): DropdownOptionWithIcon[] {
    return assets.map((asset) => {
      return {
        label: asset.getName(),
        shortLabel: asset.getSymbol(),
        value: asset.getPicassoAssetId(),
        icon: asset.getIconUrl(),
      } as DropdownOptionWithIcon;
    });
  }

  constructor(
    name: string,
    symbol: string,
    iconUrl: string,
    tokenId: TokenId,
    api?: ApiPromise
  ) {
    this.__api = api;
    this.__name = name;
    this.__symbol = symbol;
    this.__iconUrl = iconUrl;
    this.__tokenId = tokenId;
    this.__parachainAssetIds = new Map<string, BigNumber>();
    this.__existentialDeposit = new Map<SubstrateNetworkId, BigNumber | null>();
    this.__decimals = new Map<string, number>();
    this.__ratio = null;
  }

  /**
   * Returns asset id of this asset
   * on picasso parachain
   * string by default
   * @param inBn boolean
   * @returns BigNumber | string | null
   */
  getPicassoAssetId(inBn: boolean = false) {
    return this.getIdOnChain("picasso", inBn);
  }

  isSupportedOn(network: string): boolean {
    const id = this.__parachainAssetIds.get(network);
    return !!id;
  }

  getSymbol(): string {
    return this.__symbol;
  }

  getName() {
    return this.__name;
  }

  getDecimals(network: string) {
    return this.__decimals.get(network);
  }

  getRatio() {
    return this.__ratio;
  }

  getIconUrl(): string {
    return this.__iconUrl;
  }

  getTokenId(): TokenId {
    return this.__tokenId;
  }

  /**
   * Fetch balance of an account
   * @param {string} account
   * @returns {Promise<BigNumber>}
   */
  async balanceOf(account: string): Promise<BigNumber> {
    try {
      const picassoId = this.getPicassoAssetId();
      if (!this.__api || !picassoId) throw new Error("API Unavailable.");

      const _assetId = this.__api.createType(
        "CustomRpcCurrencyId",
        picassoId.toString()
      );
      const _accountId32 = this.__api.createType("AccountId32", account);
      const balance = await this.__api.rpc.assets.balanceOf(
        _assetId,
        _accountId32
      );
      return fromChainIdUnit(
        BigInt(balance.toString()),
        this.__decimals.get("picasso")
      );
    } catch (err: any) {
      console.error("[balanceOf]", err.message);
      return new BigNumber(0);
    }
  }

  /**
   * Fetch total issued amount
   * of this asset
   * @returns {Promise<BigNumber>}
   */
  async totalIssued(): Promise<BigNumber> {
    try {
      if (!this.__api) throw new Error("API Unavailable.");

      const assetId = this.__api.createType(
        "u128",
        this.getPicassoAssetId() as string
      );
      const totalIssued = await this.__api.query.tokens.totalIssuance(assetId);
      return fromChainIdUnit(BigInt(totalIssued.toString()));
    } catch (err: any) {
      return new BigNumber(0);
    }
  }

  /**
   * Set id on a different chain
   * @param {string} chainId
   * @param {BigNumber} assetId
   */
  setIdOnChain(chainId: string, assetId: BigNumber) {
    this.__parachainAssetIds.set(chainId, assetId);
  }

  /**
   * Set Asset Ratio to Pica
   */
  setRatio(ratio: AssetRatio | null) {
    this.__ratio = ratio;
  }

  setExistentialDeposit(network: SubstrateNetworkId, ed: BigNumber | null) {
    this.__existentialDeposit.set(network, ed);
  }

  /**
   * Get Asset Id on a different chain
   * @param {string} chainId
   * @param {boolean} inBn
   * @returns {BigNumber | string | null}
   */
  getIdOnChain(
    chainId: string,
    inBn: boolean = false
  ): BigNumber | string | null {
    return pipe(
      this.__parachainAssetIds.get(chainId),
      option.fromNullable,
      option.map((id) => (inBn ? id : id.toString())),
      option.fold(
        () => null,
        (a) => a
      )
    );
  }

  getExistentialDeposit(network: SubstrateNetworkId) {
    return this.__existentialDeposit.get(network);
  }

  setApi(api: ApiPromise) {
    this.__api = api;
  }

  setDecimals(network: string, decimals: number) {
    this.__decimals.set(network, decimals);
  }

  getApi(): ApiPromise {
    if (!this.__api) throw new Error("API Unavailable.");
    return this.__api;
  }

  getPrice(): BigNumber {
    return this.__price;
  }

  setPrice(price: BigNumber | string) {
    this.__price = new BigNumber(price);
  }
}

export class OwnedAsset extends Asset {
  protected __balance: BigNumber;

  static fromAsset(asset: Asset, balance: BigNumber): OwnedAsset {
    const ownedAsset = new OwnedAsset(
      asset.getName(),
      asset.getSymbol(),
      asset.getIconUrl(),
      balance,
      asset.getTokenId(),
      asset.getApi()
    );
    ownedAsset.setIdOnChain(
      "picasso",
      asset.getPicassoAssetId(true) as BigNumber
    );
    return ownedAsset;
  }

  constructor(
    name: string,
    symbol: string,
    iconUrl: string,
    balance: BigNumber,
    tokenId: TokenId,
    api?: ApiPromise
  ) {
    super(name, symbol, iconUrl, tokenId, api);

    this.__balance = balance;
  }

  getBalance(): BigNumber {
    return this.__balance;
  }
}

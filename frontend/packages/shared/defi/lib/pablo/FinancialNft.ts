import { PALLET_TYPE_ID } from "../../constants";
import { blake2AsHex } from "@polkadot/util-crypto";
import { ApiPromise } from "@polkadot/api";
import { concatU8a } from "../../u8a";
import { hexToU8a } from "@polkadot/util";
import { Asset } from "../Asset";
import BigNumber from "bignumber.js";

export class FinancialNft {
  protected __api: ApiPromise;
  protected __fNftCollectionId: BigNumber;
  protected __fNftInstanceId: BigNumber;
  static BLAKE_HASH_BIT_LENGTH = 256;

  public static async ownedFinancialNfts(
    api: ApiPromise,
    account: string
  ): Promise<FinancialNft[]> {
    try {
      const ownerInstances = await api.query.fnft.ownerInstances(account);
      const decodedInstances = ownerInstances.toJSON() as [
        number | string,
        number | string
      ][];
      return decodedInstances.map(
        ([collectionId, instanceId]) =>
          new FinancialNft(
            api,
            new BigNumber(collectionId),
            new BigNumber(instanceId)
          )
      );
    } catch (error: any) {
      console.error("[ownedFinancialNfts] ", error.message);
      return Promise.reject(error);
    }
  }

  constructor(
    api: ApiPromise,
    fNftCollectionId: BigNumber,
    fNftInstanceId: BigNumber
  ) {
    this.__api = api;
    this.__fNftCollectionId = fNftCollectionId;
    this.__fNftInstanceId = fNftInstanceId;
  }

  getAccountId(): string {
    const palletId = this.__api.consts.fnft.palletId.toU8a();
    const accountPrefix = concatU8a(PALLET_TYPE_ID, palletId);
    const tuple = this.__api.createType("(u128, u64)", [
      this.__fNftCollectionId.toString(),
      this.__fNftInstanceId.toString(),
    ]);
    const TRUNCATE_BITS = 20;
    const blakeHash = blake2AsHex(
      tuple.toU8a(),
      FinancialNft.BLAKE_HASH_BIT_LENGTH as 256
    );
    const accountId = concatU8a(
      accountPrefix,
      hexToU8a(blakeHash).subarray(0, TRUNCATE_BITS)
    );
    return this.__api.createType("AccountId32", accountId).toString();
  }

  fetchXTokenBalance(xTokenAsset: Asset): Promise<BigNumber> {
    return xTokenAsset.balanceOf(this.getAccountId());
  }
}

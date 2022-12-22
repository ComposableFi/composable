import {
  Arg,
  Field,
  InputType,
  ObjectType,
  Query,
  Resolver,
} from "type-graphql";
import type { EntityManager } from "typeorm";
import { divideBigInts } from "../../utils";
import { PabloAssetWeight, PabloPoolAsset, PabloSwap } from "../../model";

@ObjectType()
export class PabloSpotPrice {
  @Field(() => BigInt, { nullable: false })
  spotPrice!: string;

  constructor(props: Partial<PabloSpotPrice>) {
    Object.assign(this, props);
  }
}

@InputType()
export class PabloSpotPriceInput {
  @Field(() => String, { nullable: false })
  baseAssetId!: string;

  @Field(() => String, { nullable: false })
  quoteAssetId!: string;

  @Field(() => String, { nullable: false })
  poolId!: string;
}

@Resolver()
export class PabloSpotPriceResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => PabloSpotPrice)
  async pabloSpotPrice(
    @Arg("params", { validate: true }) input: PabloSpotPriceInput
  ): Promise<PabloSpotPrice> {
    const { baseAssetId, quoteAssetId, poolId } = input;

    const manager = await this.tx();

    const swap1 = await manager.getRepository(PabloSwap).findOne({
      where: {
        baseAssetId,
        quoteAssetId,
        pool: {
          id: poolId,
        },
      },
    });

    const swap2 = await manager.getRepository(PabloSwap).findOne({
      where: {
        baseAssetId: quoteAssetId,
        quoteAssetId: baseAssetId,
        pool: {
          id: poolId,
        },
      },
    });

    const timestamp1 = swap1?.timestamp;
    const timestamp2 = swap2?.timestamp;

    let swap: PabloSwap;

    if (timestamp1 && !timestamp2) {
      swap = swap1;
    } else if (!timestamp1 && timestamp2) {
      swap = swap2;
    } else if (timestamp1 && timestamp2) {
      swap = timestamp1 > timestamp2 ? swap1 : swap2;
    } else {
      const baseAsset = await manager.getRepository(PabloPoolAsset).findOne({
        where: {
          assetId: baseAssetId,
          pool: {
            id: poolId,
          },
        },
      });

      const quoteAsset = await manager.getRepository(PabloPoolAsset).findOne({
        where: {
          assetId: quoteAssetId,
          pool: {
            id: poolId,
          },
        },
      });

      if (!baseAsset || !quoteAsset) {
        throw new Error(
          "No liquidity data for this pool. Can't compute spot price."
        );
      }

      const baseAssetWeight = await manager
        .getRepository(PabloAssetWeight)
        .findOne({
          where: {
            assetId: baseAssetId,
            pool: {
              id: poolId,
            },
          },
        });

      const quoteAssetWeight = await manager
        .getRepository(PabloAssetWeight)
        .findOne({
          where: {
            assetId: quoteAssetId,
            pool: {
              id: poolId,
            },
          },
        });

      const weightRatio =
        baseAssetWeight?.weight && quoteAssetWeight?.weight
          ? baseAssetWeight.weight / quoteAssetWeight.weight
          : 1;

      return {
        spotPrice: (
          divideBigInts(quoteAsset.totalLiquidity, baseAsset.totalLiquidity) *
          weightRatio
        ).toString(),
      };
    }

    const spotPrice =
      baseAssetId === swap.baseAssetId
        ? swap.spotPrice
        : (1 / Number(swap.spotPrice)).toString();

    return {
      spotPrice,
    };
  }
}

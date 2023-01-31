import { Arg, Field, InputType, ObjectType, Query, Resolver } from "type-graphql";
import type { EntityManager } from "typeorm";
import { PabloPool } from "../../model";
import { getRange } from "./common";
import { getSpotPrice } from "../../dbHelper";

@ObjectType()
export class SpotPriceHistory {
  @Field(() => String, { nullable: false })
  date!: string;

  @Field(() => Number, { nullable: false })
  spotPrice!: number;

  constructor(props: Partial<SpotPriceHistory>) {
    Object.assign(this, props);
  }
}

@ObjectType()
export class PabloSpotPriceChart {
  @Field(() => String, { nullable: false })
  assetId!: string;

  @Field(() => [SpotPriceHistory], { nullable: false })
  history!: SpotPriceHistory[];

  constructor(props: PabloSpotPriceChart) {
    Object.assign(this, props);
  }
}

@InputType()
export class PabloSpotPriceChartInput {
  @Field(() => String, { nullable: false })
  range!: string;

  @Field(() => String, { nullable: false })
  poolId!: string;
}

@Resolver()
export class PabloSpotPriceChartResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => [PabloSpotPriceChart])
  async pabloSpotPriceChart(
    @Arg("params", { validate: true }) input: PabloSpotPriceChartInput
  ): Promise<PabloSpotPriceChart[]> {
    const { range, poolId } = input;

    const manager = await this.tx();

    const pool = await manager.getRepository(PabloPool).findOne({
      where: { id: poolId.toString() },
      relations: {
        poolAssets: true,
        poolAssetWeights: true
      }
    });

    if (!pool) {
      throw new Error(`Pool with id ${poolId} not found`);
    }

    const assetIds = pool.poolAssets.map(asset => asset.assetId);

    const timestamps = getRange(range);

    const { quoteAssetId } = pool;
    const baseAssetId = assetIds.find(id => id !== quoteAssetId)!;

    // Quote
    const quoteHistory: Array<SpotPriceHistory> = [];
    for (const timestamp of timestamps) {
      const time = timestamp.toISOString();
      const spotPrice = await getSpotPrice(manager, baseAssetId, quoteAssetId, poolId, new Date(time).getTime());
      const spotPriceHistory = new SpotPriceHistory({
        date: time,
        spotPrice
      });
      quoteHistory.push(spotPriceHistory);
    }

    // Base
    const baseHistory: Array<SpotPriceHistory> = [];
    for (const timestamp of timestamps) {
      const time = timestamp.toISOString();
      const spotPrice = await getSpotPrice(manager, quoteAssetId, baseAssetId, poolId, new Date(time).getTime());
      const spotPriceHistory = new SpotPriceHistory({
        date: time,
        spotPrice
      });
      baseHistory.push(spotPriceHistory);
    }

    return [
      {
        assetId: quoteAssetId,
        history: quoteHistory
      },
      {
        assetId: baseAssetId,
        history: baseHistory
      }
    ];
  }
}

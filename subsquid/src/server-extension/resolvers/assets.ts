import { Field, ObjectType, Query, Resolver } from "type-graphql";
import type { EntityManager } from "typeorm";
import { Asset, HistoricalAssetPrice } from "../../model";

@ObjectType()
export class Assets {
  @Field(() => String, { nullable: false })
  id!: string;

  @Field(() => String, { nullable: false })
  eventId!: string;

  @Field(() => BigInt, { nullable: false })
  price!: bigint;

  @Field(() => BigInt, { nullable: false })
  prevPrice!: bigint;

  @Field(() => Number, { nullable: false })
  change!: number;

  constructor(props: Partial<Assets>) {
    Object.assign(this, props);
  }
}

type AssetPrice = {
  id: string;
  price: string;
};

type HistoricalPrice = {
  id: string;
  event_id: string;
  asset_id: string;
  price: string;
  timestamp: string;
};

type AssetData = {
  id: string;
  eventId: string;
  price: bigint;
  prevPrice: bigint;
  timestamp: string;
  change: number;
};

@Resolver()
export class AssetsResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => [Assets])
  async assetsPrices(): Promise<Assets[]> {
    const DAY_IN_MS = 24 * 60 * 60 * 1_000;
    const currentTimestamp = new Date().valueOf();
    const threshold = currentTimestamp - DAY_IN_MS;

    const manager = await this.tx();

    // Query asset prices.
    const assets: AssetPrice[] = await manager.getRepository(Asset).query(
      `
        SELECT
            id,
            price
        FROM asset
      `
    );

    // Get asset prices into an object for easier read.
    let assetsPrices = assets.reduce<Record<string, string>>((acc, curr) => {
      acc[curr.id] = curr.price;
      return acc;
    }, {});

    // Query the closest historical prices for each asset that are older than the threshold (24h).
    const historicalAssetPrices: HistoricalPrice[] = await manager
      .getRepository(HistoricalAssetPrice)
      .query(
        `
            SELECT *
            FROM historical_asset_price
            WHERE timestamp IN (
                SELECT
                    MAX(timestamp) as max_timestamp
                FROM historical_asset_price
                WHERE timestamp < ${threshold}
                GROUP BY asset_id
            )
        `
      );

    // Prepare data and format for returning.
    const historicalPrices = historicalAssetPrices.reduce<
      Record<string, AssetData>
    >((acc, curr) => {
      const price = BigInt(assetsPrices?.[curr.asset_id] || curr.price);
      const prevPrice = BigInt(curr.price);
      acc[curr.asset_id] = {
        id: curr.asset_id,
        eventId: curr.event_id,
        timestamp: curr.timestamp,
        price,
        prevPrice,
        change: Number((100n * (price - prevPrice)) / prevPrice),
      };

      return acc;
    }, {});

    return Object.values(historicalPrices);
  }
}

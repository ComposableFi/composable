import {
  Arg,
  Field,
  InputType,
  ObjectType,
  Query,
  Resolver,
} from "type-graphql";
import type { EntityManager } from "typeorm";
import { HistoricalLockedValue, PabloPool } from "../../model";
import { getRange } from "./common";

@ObjectType()
export class PabloTVL {
  @Field(() => String, { nullable: false })
  date!: string;

  @Field(() => BigInt, { nullable: false })
  totalValueLocked!: bigint;

  @Field(() => String, { nullable: false })
  assetId!: string;

  constructor(props: Partial<PabloTVL>) {
    Object.assign(this, props);
  }
}

@InputType()
export class PabloTVLInput {
  @Field(() => String, { nullable: false })
  range!: string;

  @Field(() => String, { nullable: false })
  poolId!: string;
}

@Resolver()
export class PabloTVLResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => [PabloTVL])
  async pabloTVL(
    @Arg("params", { validate: true }) input: PabloTVLInput
  ): Promise<PabloTVL[]> {
    const { range, poolId } = input;

    const manager = await this.tx();

    const pool = await manager.getRepository(PabloPool).findOne({
      where: { id: poolId.toString() },
      order: { timestamp: "DESC" },
      relations: {
        poolAssets: true,
        poolAssetWeights: true,
      },
    });

    if (!pool) {
      throw new Error(`Pool with id ${poolId} not found`);
    }

    const timestamps = getRange(range);
    // Map timestamp to tvl
    const lockedValues: Record<string, bigint> = {};

    for (const timestamp of timestamps) {
      const time = timestamp.toISOString();
      const row = await manager
        .getRepository(HistoricalLockedValue)
        .createQueryBuilder()
        .select(`'${time}'`, "date")
        .addSelect("asset_id", "assetId")
        .addSelect(
          `coalesce(tvl('${time}', 'Pablo', '${pool.baseAssetId}'), 0)`,
          "totalValueLocked"
        )
        .getRawOne();

      lockedValues[time] = row.totalValueLocked;
    }

    return Object.keys(lockedValues).map((date) => {
      return new PabloTVL({
        date,
        totalValueLocked: lockedValues[date],
        assetId: pool.baseAssetId,
      });
    });
  }
}

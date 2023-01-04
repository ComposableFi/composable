import {
  Arg,
  Field,
  InputType,
  ObjectType,
  Query,
  Resolver,
} from "type-graphql";
import type { EntityManager } from "typeorm";
import { IsEnum } from "class-validator";
import { HistoricalLockedValue, LockedSource, PabloPool } from "../../model";
import { getStartAndStep } from "./common";

@ObjectType()
export class PabloTVL {
  @Field(() => String, { nullable: false })
  date!: string;

  @Field(() => String, { nullable: false })
  source!: string;

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
  @IsEnum(LockedSource, {
    message:
      "Value must be a LockedSource enum. For example, 'Pablo', 'VestingSchedules', 'StakingRewards', etc",
  })
  source!: LockedSource;

  @Field(() => String, { nullable: false })
  poolId!: string;
}

@Resolver()
export class PabloTVLResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => [PabloTVL])
  async totalValueLocked(
    @Arg("params", { validate: true }) input: PabloTVLInput
  ): Promise<PabloTVL[]> {
    const { range, source, poolId } = input;

    const manager = await this.tx();

    const { startHoursAgo, step } = getStartAndStep(range);

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

    const rows: { period: string; total_value_locked: string }[] = await manager
      .getRepository(HistoricalLockedValue)
      .query(
        `
          WITH range AS (
            SELECT
              generate_series (
                ${startHoursAgo},
                0,
                ${-step}
              ) AS hour
          )
          SELECT
              date_trunc('hour', current_timestamp) - hour * interval '1 hour' as period,
              coalesce(hourly_total_value_locked(hour, '${source}', '${poolId}'), 0) as total_value_locked
          FROM range
          UNION
          (SELECT
              CURRENT_TIMESTAMP as period,
              COALESCE(accumulated_amount, 0) as total_value_locked
          FROM historical_locked_value
          WHERE source = '${source}'
          AND source_entity_id = '${poolId}'
          ORDER BY timestamp DESC
          LIMIT 1)
          ORDER BY period;
          `
      );

    return rows.map(
      (row) =>
        new PabloTVL({
          date: new Date(row.period).toISOString(),
          totalValueLocked: BigInt(row.total_value_locked),
          source,
          assetId: pool?.poolAssets[0].assetId || "",
        })
    );
  }
}

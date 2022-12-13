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
import { HistoricalLockedValue, LockedSource } from "../../model";
import { getStartAndStep } from "./common";

@ObjectType()
export class TotalValueLocked {
  @Field(() => String, { nullable: false })
  date!: string;

  @Field(() => String, { nullable: false })
  source!: string;

  @Field(() => BigInt, { nullable: false })
  totalValueLocked!: bigint;

  constructor(props: Partial<TotalValueLocked>) {
    Object.assign(this, props);
  }
}

@InputType()
export class TotalValueLockedInput {
  @Field(() => String, { nullable: false })
  range!: string;

  @Field(() => String, { nullable: true, defaultValue: LockedSource.All })
  @IsEnum(LockedSource, {
    message:
      "Value must be a LockedSource enum. For example, 'All', 'Pablo', 'VestingSchedules', 'StakingRewards', etc",
  })
  source?: LockedSource;
}

@Resolver()
export class TotalValueLockedResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => [TotalValueLocked])
  async totalValueLocked(
    @Arg("params", { validate: true }) input: TotalValueLockedInput
  ): Promise<TotalValueLocked[]> {
    const { range, source } = input;

    const manager = await this.tx();

    const { startHoursAgo, step } = getStartAndStep(range);

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
              coalesce(hourly_total_value_locked(hour, '${source}'), 0) as total_value_locked
          FROM range
          UNION
          (SELECT
              CURRENT_TIMESTAMP as period,
              COALESCE(amount, 0) as total_value_locked
          FROM historical_locked_value
          WHERE source = '${source}'
          ORDER BY period DESC
          LIMIT 1)
          ORDER BY period;
          `
      );

    return rows.map(
      (row) =>
        new TotalValueLocked({
          date: new Date(row.period).toISOString(),
          totalValueLocked: BigInt(row.total_value_locked),
          source,
        })
    );
  }
}

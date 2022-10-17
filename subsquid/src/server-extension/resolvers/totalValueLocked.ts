import {
  Arg,
  Field,
  InputType,
  Int,
  ObjectType,
  Query,
  Resolver,
} from "type-graphql";
import type { EntityManager } from "typeorm";
import { IsDateString, IsEnum, Min } from "class-validator";
import { HistoricalLockedValue, LockedSource } from "../../model";
import { getTimelineParams } from "./common";

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
  @Field(() => Int, { nullable: false })
  @Min(1)
  intervalMinutes!: number;

  @Field(() => String, { nullable: true })
  @IsDateString()
  dateFrom?: string;

  @Field(() => String, { nullable: true })
  @IsDateString()
  dateTo?: string;

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
    const { intervalMinutes, dateFrom, dateTo, source } = input;
    const { where, params } = getTimelineParams(
      intervalMinutes,
      dateFrom,
      dateTo
    );

    const manager = await this.tx();

    const rows: {
      period: string;
      total_value_locked: string;
      source: string;
    }[] = await manager.getRepository(HistoricalLockedValue).query(
      `
            SELECT
              round(timestamp / $1) * $1 as period,
              max(amount) as total_value_locked
            FROM historical_locked_value
            WHERE ${where.join(" AND ")}
            AND source = '${source}'
            GROUP BY period
            ORDER BY period DESC
        `,
      params
    );

    return rows.map(
      (row) =>
        new TotalValueLocked({
          date: new Date(parseInt(row.period, 10)).toISOString(),
          totalValueLocked: BigInt(row.total_value_locked),
          source,
        })
    );
  }
}

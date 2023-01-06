import { Arg, Field, InputType, ObjectType, Query, Resolver } from "type-graphql";
import type { EntityManager } from "typeorm";
import { HistoricalVolume } from "../../model";
import { getStartAndStep } from "./common";

@ObjectType()
export class TotalVolume {
  @Field(() => String, { nullable: false })
  date!: string;

  @Field(() => BigInt, { nullable: false })
  totalVolume!: bigint;

  constructor(props: Partial<TotalVolume>) {
    Object.assign(this, props);
  }
}

@InputType()
export class TotalVolumeInput {
  @Field(() => String, { nullable: false })
  range!: string;
}

@Resolver()
export class TotalVolumeResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => [TotalVolume])
  async totalVolume(@Arg("params", { validate: true }) input: TotalVolumeInput): Promise<TotalVolume[]> {
    const { range } = input;

    const { startHoursAgo, step } = getStartAndStep(range);

    const manager = await this.tx();

    const rows: {
      period: string;
      total_volume: string;
    }[] = await manager.getRepository(HistoricalVolume).query(
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
          coalesce(hourly_total_volume(hour), 0) as total_volume
        FROM range
        UNION
        (SELECT
          CURRENT_TIMESTAMP as period,
          COALESCE(amount, 0) as total_volume
        FROM historical_volume
        ORDER BY period DESC
        LIMIT 1)
        ORDER BY period;
      `
    );

    return rows.map(
      row =>
        new TotalVolume({
          date: new Date(row.period).toISOString(),
          totalVolume: BigInt(row.total_volume)
        })
    );
  }
}

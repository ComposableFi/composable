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
import { IsDateString, Min } from "class-validator";
import { HistoricalVolume } from "../../model";
import { getTimelineParams } from "./common";

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
  @Field(() => Int, { nullable: false })
  @Min(1)
  intervalMinutes!: number;

  @Field(() => String, { nullable: true })
  @IsDateString()
  dateFrom?: string;

  @Field(() => String, { nullable: true })
  @IsDateString()
  dateTo?: string;
}

@Resolver()
export class TotalVolumeResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => [TotalVolume])
  async totalVolume(
    @Arg("params", { validate: true }) input: TotalVolumeInput
  ): Promise<TotalVolume[]> {
    const { intervalMinutes, dateFrom, dateTo } = input;
    const { where, params } = getTimelineParams(
      intervalMinutes,
      dateFrom,
      dateTo
    );

    const manager = await this.tx();

    const rows: {
      period: string;
      total_volume: string;
    }[] = await manager.getRepository(HistoricalVolume).query(
      `
            SELECT
              round(timestamp / $1) * $1 as period,
              max(amount) as total_volume
            FROM historical_volume
            WHERE ${where.join(" AND ")}
            GROUP BY period
            ORDER BY period DESC
        `,
      params
    );

    return rows.map(
      (row) =>
        new TotalVolume({
          date: new Date(parseInt(row.period, 10)).toISOString(),
          totalVolume: BigInt(row.total_volume),
        })
    );
  }
}

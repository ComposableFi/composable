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
import { Activity } from "../../model";
import { getTimelineParams } from "./common";

@ObjectType()
export class ActiveUsers {
  @Field(() => String, { nullable: false })
  date!: string;

  @Field(() => Number, { nullable: false })
  count!: number;

  constructor(props: Partial<ActiveUsers>) {
    Object.assign(this, props);
  }
}

@InputType()
export class ActiveUsersInput {
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
export class ActiveUsersResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => [ActiveUsers])
  async activeUsers(
    @Arg("params", { validate: true }) input: ActiveUsersInput
  ): Promise<ActiveUsers[]> {
    const { intervalMinutes, dateFrom, dateTo } = input;
    const { where, params } = getTimelineParams(
      intervalMinutes,
      dateFrom,
      dateTo
    );

    const manager = await this.tx();

    const rows: { period: string; count: string }[] = await manager
      .getRepository(Activity)
      .query(
        `
            SELECT
              round(timestamp / $1) * $1 as period,
              count(distinct account_id) as count
            FROM activity
            WHERE ${where.join(" AND ")}
            GROUP BY period
            ORDER BY period DESC
        `,
        params
      );

    return rows.map(
      (row) =>
        new ActiveUsers({
          date: new Date(parseInt(row.period, 10)).toISOString(),
          count: Number(row.count),
        })
    );
  }
}

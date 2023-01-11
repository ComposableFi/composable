import { Arg, Field, InputType, ObjectType, Query, Resolver } from "type-graphql";
import type { EntityManager } from "typeorm";
import { Activity } from "../../model";

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
  @Field(() => String, { nullable: false })
  range!: string;
}

@Resolver()
export class ActiveUsersResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => [ActiveUsers])
  async activeUsers(@Arg("params", { validate: true }) input: ActiveUsersInput): Promise<ActiveUsers[]> {
    const { range } = input;

    if (range !== "day" && range !== "week" && range !== "month" && range !== "year") {
      throw new Error("Invalid range. It should be 'day', 'week', 'month' or 'year'.");
    }

    const manager = await this.tx();

    let rows: { period: string; count: string }[] = [];

    // Hourly
    switch (range) {
      case "day": {
        const startHoursAgo = 22;

        rows = await manager.getRepository(Activity).query(
          `
          WITH range AS (
            SELECT
               generate_series (
                 ${startHoursAgo},
                 0,
                 -1
               ) AS hour
          )
          SELECT
              date_trunc('hour', current_timestamp) - hour * interval '1 hour' as period, 
              hourly_active_users(hour) as count
          FROM range
          UNION
          SELECT 
            current_timestamp as period,
            count(distinct account_id)
          FROM activity
          WHERE
            timestamp > current_timestamp - interval '1 day'
          ORDER BY period;
        `
        );
        break;
      }
      case "week":
      case "month": {
        const startDaysAgo = range === "week" ? 5 : 28;

        rows = await manager.getRepository(Activity).query(
          `
          WITH range AS (
            SELECT
               generate_series (
                 ${startDaysAgo},
                 0,
                 -1
               ) AS day
            )
            SELECT
                date_trunc('day', current_timestamp) - day * interval '1 day' as period,
                daily_active_users(day, 1) as count
            FROM range
            UNION
            SELECT
                 current_timestamp as period,
                 COUNT(DISTINCT account_id) as count
            FROM activity
            WHERE timestamp >= current_timestamp - interval '1 day'
            ORDER BY period;
        `
        );
        break;
      }
      case "year":
      default: {
        const startMonthsAgo = 10;

        rows = await manager.getRepository(Activity).query(
          `
          WITH range AS (
            SELECT
               generate_series (
                 ${startMonthsAgo},
                 0,
                 -1
               ) AS month
            )
            SELECT
                date_trunc('month', current_timestamp - month * interval '1 month') as period, 
                daily_active_users(month, 30) as count
            FROM range
            UNION
            SELECT
                current_timestamp as period,
                COUNT(DISTINCT account_id) as count
            FROM activity
            WHERE timestamp >= current_timestamp - interval '1 month'
            ORDER BY period;
        `
        );
        break;
      }
    }

    return rows.map(
      row =>
        new ActiveUsers({
          date: row.period,
          count: Number(row.count)
        })
    );
  }
}

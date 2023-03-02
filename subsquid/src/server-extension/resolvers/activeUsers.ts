import { Arg, Field, InputType, ObjectType, Query, Resolver } from "type-graphql";
import type { EntityManager } from "typeorm";
import { IsEnum } from "class-validator";
import { Activity } from "../../model";
import { getRange } from "./common";
import { DAY_IN_MS } from "../../constants";

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
  @IsEnum(["day", "week", "month", "year"])
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

    const timestamps = getRange(range);

    const activeUsers: Array<ActiveUsers> = [];

    for (const timestamp of timestamps) {
      const { count } = await manager
        .getRepository(Activity)
        .createQueryBuilder()
        .select("COUNT(DISTINCT(account_id))", "count")
        .where(`timestamp >= :timestampFrom`, { timestampFrom: new Date(timestamp.getTime() - DAY_IN_MS) })
        .andWhere(`timestamp < :timestampTo`, { timestampTo: new Date(timestamp.getTime()) })
        .getRawOne();

      activeUsers.push(
        new ActiveUsers({
          date: timestamp.toISOString(),
          count: count || 0
        })
      );
    }

    return activeUsers;
  }
}

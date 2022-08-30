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
import { PicassoStakingPosition } from "../../model";

@ObjectType()
export class TotalValueLocked {
  @Field(() => String, { nullable: false })
  date!: string;

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
}

@Resolver()
export class TotalValueLockedResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => [TotalValueLocked])
  async picassoTotalValueLocked(
    @Arg("params", { validate: true }) input: TotalValueLockedInput
  ): Promise<TotalValueLocked[]> {
    const intervalMilliseconds = input.intervalMinutes * 60 * 1000;
    const params: any[] = [intervalMilliseconds];
    const where: string[] = [];
    let from: number;

    // Set "from" filter
    if (input.dateFrom) {
      from = new Date(input.dateFrom).valueOf();
    } else {
      from = 0;
    }
    from = Math.floor(from / intervalMilliseconds) * intervalMilliseconds;
    where.push(`timestamp > $${params.push(from)}`);

    // Set "to" filter
    if (input.dateTo) {
      let to = new Date(input.dateTo).valueOf();
      to = Math.ceil(to / intervalMilliseconds) * intervalMilliseconds;
      where.push(`timestamp < $${params.push(to)}`);
    }

    const manager = await this.tx();

    let rows: { period: string; total_value_locked: string }[] = await manager
      .getRepository(PicassoStakingPosition)
      .query(
        `
            SELECT
              round(timestamp / $1) * $1 as period,
              sum(amount) as total_value_locked
            FROM historical_locked_value
            WHERE ${where.join(" AND ")}
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
        })
    );
  }
}

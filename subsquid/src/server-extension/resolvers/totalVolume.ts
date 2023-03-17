import { Arg, Field, InputType, ObjectType, Query, Resolver } from "type-graphql";
import { LessThan } from "typeorm";
import type { EntityManager } from "typeorm";
import { IsEnum } from "class-validator";
import { HistoricalVolume } from "../../model";
import { getVolumeRange } from "./common";

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
  @IsEnum(["now", "month", "year"])
  range!: string;
}

@Resolver()
export class TotalVolumeResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => [TotalVolume])
  async totalVolume(@Arg("params", { validate: true }) input: TotalVolumeInput): Promise<TotalVolume[]> {
    const { range } = input;
    const manager = await this.tx();

    const timestamps = getVolumeRange(range);

    const totalVolumes: Array<TotalVolume> = [];

    for (const timestamp of timestamps) {
      const historicalVolume = await manager.getRepository(HistoricalVolume).findOne({
        where: {
          timestamp: LessThan(new Date(timestamp.getTime()))
        },
        order: {
          timestamp: "DESC"
        }
      });

      const accumulatedAmount = historicalVolume?.accumulatedAmount ?? 0n;

      totalVolumes.push(
        new TotalVolume({
          date: timestamp.toISOString(),
          totalVolume: accumulatedAmount
        })
      );
    }

    return totalVolumes;
  }
}

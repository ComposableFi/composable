import { Arg, Field, InputType, ObjectType, Query, Resolver } from "type-graphql";
import type { EntityManager } from "typeorm";
import { LessThan, MoreThan, And } from "typeorm";
import { PabloSwap } from "../../model";
import { DAY_IN_MS, getVolumeRange } from "./common";

@ObjectType()
class AssetIdAmount {
  @Field(() => String, { nullable: false })
  assetId!: string;

  @Field(() => BigInt, { nullable: false })
  amount!: bigint;

  constructor(props: AssetIdAmount) {
    Object.assign(this, props);
  }
}

@ObjectType()
export class PabloTotalVolume {
  @Field(() => String, { nullable: false })
  date!: string;

  @Field(() => [AssetIdAmount], { nullable: false })
  volumes!: AssetIdAmount[];

  constructor(props: Partial<PabloTotalVolume>) {
    Object.assign(this, props);
  }
}

@InputType()
export class PabloTotalVolumeInput {
  @Field(() => String, { nullable: false })
  range!: string;

  @Field(() => String, { nullable: true })
  poolId!: string;
}

@Resolver()
export class PabloTotalVolumeResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => [PabloTotalVolume])
  async pabloTotalVolume(@Arg("params", { validate: true }) input: PabloTotalVolumeInput): Promise<PabloTotalVolume[]> {
    const { range, poolId } = input;

    const manager = await this.tx();

    const timestamps = getVolumeRange(range);
    // Map timestamp to volume
    const volumes: Record<string, AssetIdAmount[]> = {};

    for (const timestamp of timestamps) {
      const time = timestamp.toISOString();

      const swaps = await manager.getRepository(PabloSwap).find({
        where: {
          timestamp: And(
            LessThan(new Date(timestamp.getTime())),
            MoreThan(new Date(timestamp.getTime() - (range === "year" ? 7 : 1) * DAY_IN_MS))
          ),
          ...(poolId
            ? {
                pool: {
                  id: poolId
                }
              }
            : {})
        }
      });

      const currVolumes = swaps.reduce<Record<string, bigint>>((acc, swap) => {
        acc[swap.quoteAssetId] = (acc[swap.quoteAssetId] || 0n) + swap.quoteAssetAmount;
        return acc;
      }, {});

      volumes[time] = Object.keys(currVolumes).map(
        assetId =>
          new AssetIdAmount({
            assetId,
            amount: currVolumes[assetId]
          })
      );
    }

    return Object.keys(volumes).map(date => {
      return new PabloTotalVolume({
        date,
        volumes: volumes[date]
      });
    });
  }
}

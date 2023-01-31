import { Arg, Field, InputType, ObjectType, Query, Resolver } from "type-graphql";
import type { EntityManager } from "typeorm";
import { getSpotPrice } from "../../dbHelper";

@ObjectType()
export class PabloSpotPrice {
  @Field(() => BigInt, { nullable: false })
  spotPrice!: string;

  constructor(props: Partial<PabloSpotPrice>) {
    Object.assign(this, props);
  }
}

@InputType()
export class PabloSpotPriceInput {
  @Field(() => String, { nullable: false })
  baseAssetId!: string;

  @Field(() => String, { nullable: false })
  quoteAssetId!: string;

  @Field(() => String, { nullable: false })
  poolId!: string;
}

@Resolver()
export class PabloSpotPriceResolver {
  constructor(private tx: () => Promise<EntityManager>) {}

  @Query(() => PabloSpotPrice)
  async pabloSpotPrice(@Arg("params", { validate: true }) input: PabloSpotPriceInput): Promise<PabloSpotPrice> {
    const { baseAssetId, quoteAssetId, poolId } = input;

    const manager = await this.tx();

    const spotPrice = await getSpotPrice(manager, baseAssetId, quoteAssetId, poolId);

    return {
      spotPrice: spotPrice.toString()
    };
  }
}

import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {RewardRatePeriod} from "./_rewardRatePeriod"

@Entity_()
export class StakingRewardsPool {
    constructor(props?: Partial<StakingRewardsPool>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @Column_("text", {nullable: false})
    eventId!: string

    @Index_()
    @Column_("text", {nullable: false})
    poolId!: string

    /**
     * Last updated block id
     */
    @Column_("text", {nullable: false})
    blockId!: string

    @Index_()
    @Column_("text", {nullable: false})
    owner!: string

    @Index_()
    @Column_("text", {nullable: false})
    assetId!: string

    @Index_()
    @Column_("text", {nullable: false})
    shareAssetId!: string

    @Index_()
    @Column_("text", {nullable: false})
    financialNftAssetId!: string

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    minimumStakingAmount!: bigint

    @Column_("int4", {nullable: false})
    startBlock!: number

    @Column_("varchar", {length: 9, nullable: false})
    rewardRatePeriod!: RewardRatePeriod

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    rewardRateAmount!: bigint
}

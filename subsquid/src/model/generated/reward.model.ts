import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {RewardPool} from "./rewardPool.model"
import {RewardRatePeriod} from "./_rewardRatePeriod"

@Entity_()
export class Reward {
    constructor(props?: Partial<Reward>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => RewardPool, {nullable: true})
    rewardPool!: RewardPool

    @Column_("varchar", {length: 10, nullable: false})
    rewardRatePeriod!: RewardRatePeriod

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    rewardRateAmount!: bigint
}

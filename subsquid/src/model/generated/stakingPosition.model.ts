import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_, OneToOne as OneToOne_, JoinColumn as JoinColumn_} from "typeorm"
import * as marshal from "./marshal"
import {RewardPool} from "./rewardPool.model"
import {Event} from "./event.model"
import {LockedSource} from "./_lockedSource"

@Index_(["fnftCollectionId", "fnftInstanceId"], {unique: true})
@Entity_()
export class StakingPosition {
    constructor(props?: Partial<StakingPosition>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => RewardPool, {nullable: true})
    rewardPool!: RewardPool

    @Index_({unique: true})
    @OneToOne_(() => Event, {nullable: false})
    @JoinColumn_()
    event!: Event

    /**
     * Unique identifier for the position on chain
     */
    @Column_("text", {nullable: false})
    fnftCollectionId!: string

    @Index_()
    @Column_("text", {nullable: false})
    fnftInstanceId!: string

    @Index_()
    @Column_("text", {nullable: false})
    owner!: string

    @Column_("text", {nullable: false})
    assetId!: string

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    amount!: bigint

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    startTimestamp!: bigint

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    duration!: bigint

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: true})
    endTimestamp!: bigint | undefined | null

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    rewardMultiplier!: bigint

    @Column_("varchar", {length: 16, nullable: false})
    source!: LockedSource

    @Column_("bool", {nullable: false})
    removed!: boolean
}

import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, OneToOne as OneToOne_, Index as Index_, JoinColumn as JoinColumn_, OneToMany as OneToMany_} from "typeorm"
import {Event} from "./event.model"
import {StakingPosition} from "./stakingPosition.model"
import {Reward} from "./reward.model"

@Entity_()
export class RewardPool {
    constructor(props?: Partial<RewardPool>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_({unique: true})
    @OneToOne_(() => Event, {nullable: false})
    @JoinColumn_()
    event!: Event

    @Index_()
    @Column_("text", {nullable: false})
    assetId!: string

    /**
     * Last updated block id
     */
    @Column_("text", {nullable: false})
    blockId!: string

    @OneToMany_(() => StakingPosition, e => e.rewardPool)
    stakingPositions!: StakingPosition[]

    @OneToMany_(() => Reward, e => e.rewardPool)
    rewards!: Reward[]
}

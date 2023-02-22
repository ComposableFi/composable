import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {Event} from "./event.model"

@Entity_()
export class HistoricalStakingApr {
    constructor(props?: Partial<HistoricalStakingApr>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => Event, {nullable: true})
    event!: Event

    @Index_()
    @Column_("text", {nullable: false})
    assetId!: string

    @Column_("numeric", {transformer: marshal.floatTransformer, nullable: false})
    stakingApr!: number

    @Index_()
    @Column_("timestamp with time zone", {nullable: false})
    timestamp!: Date

    /**
     * Last updated block id
     */
    @Column_("text", {nullable: false})
    blockId!: string
}

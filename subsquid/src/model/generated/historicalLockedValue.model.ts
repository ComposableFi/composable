import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {Event} from "./event.model"
import {LockedSource} from "./_lockedSource"

@Entity_()
export class HistoricalLockedValue {
    constructor(props?: Partial<HistoricalLockedValue>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => Event, {nullable: true})
    event!: Event

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    amount!: bigint

    /**
     * Total amount of locked value
     */
    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    accumulatedAmount!: bigint

    @Index_()
    @Column_("timestamp with time zone", {nullable: false})
    timestamp!: Date

    @Column_("varchar", {length: 16, nullable: false})
    source!: LockedSource

    @Index_()
    @Column_("text", {nullable: false})
    assetId!: string

    /**
     * ID of the entity that locked the value (e.g. Pablo pool id)
     */
    @Index_()
    @Column_("text", {nullable: true})
    sourceEntityId!: string | undefined | null

    /**
     * Last updated block id
     */
    @Column_("text", {nullable: false})
    blockId!: string
}

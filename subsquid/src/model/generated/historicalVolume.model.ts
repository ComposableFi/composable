import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {Event} from "./event.model"
import {LockedSource} from "./_lockedSource"
import {PabloPool} from "./pabloPool.model"

@Entity_()
export class HistoricalVolume {
    constructor(props?: Partial<HistoricalVolume>) {
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
     * Total volume
     */
    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    accumulatedAmount!: bigint

    @Index_()
    @Column_("timestamp with time zone", {nullable: false})
    timestamp!: Date

    @Index_()
    @Column_("text", {nullable: false})
    assetId!: string

    @Column_("varchar", {length: 16, nullable: false})
    source!: LockedSource

    @Index_()
    @ManyToOne_(() => PabloPool, {nullable: true})
    pool!: PabloPool

    /**
     * Last updated block id
     */
    @Column_("text", {nullable: false})
    blockId!: string
}

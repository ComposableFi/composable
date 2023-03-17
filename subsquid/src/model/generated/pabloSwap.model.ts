import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {Event} from "./event.model"
import {PabloPool} from "./pabloPool.model"
import {PabloFee} from "./pabloFee.model"

@Entity_()
export class PabloSwap {
    constructor(props?: Partial<PabloSwap>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => Event, {nullable: true})
    event!: Event | undefined | null

    @Index_()
    @ManyToOne_(() => PabloPool, {nullable: true})
    pool!: PabloPool

    @Index_()
    @Column_("text", {nullable: false})
    baseAssetId!: string

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    baseAssetAmount!: bigint

    @Index_()
    @Column_("text", {nullable: false})
    quoteAssetId!: string

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    quoteAssetAmount!: bigint

    @Column_("text", {nullable: false})
    spotPrice!: string

    @Index_()
    @ManyToOne_(() => PabloFee, {nullable: true})
    fee!: PabloFee

    @Index_()
    @Column_("timestamp with time zone", {nullable: false})
    timestamp!: Date

    /**
     * Last updated block id
     */
    @Column_("text", {nullable: false})
    blockId!: string

    @Column_("bool", {nullable: false})
    success!: boolean
}

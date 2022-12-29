import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {Event} from "./event.model"
import {PabloPool} from "./pabloPool.model"

@Entity_()
export class PabloFee {
    constructor(props?: Partial<PabloFee>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => Event, {nullable: true})
    event!: Event

    @Index_()
    @ManyToOne_(() => PabloPool, {nullable: true})
    pool!: PabloPool

    @Index_()
    @Column_("text", {nullable: false})
    assetId!: string

    @Index_()
    @Column_("text", {nullable: false})
    account!: string

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    fee!: bigint

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    lpFee!: bigint

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    ownerFee!: bigint

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    protocolFee!: bigint

    @Index_()
    @Column_("timestamp with time zone", {nullable: false})
    timestamp!: Date

    /**
     * Last updated block id
     */
    @Column_("text", {nullable: false})
    blockId!: string
}

import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import {PabloPool} from "./pabloPool.model"
import {Event} from "./event.model"
import {PabloTx} from "./_pabloTx"
import {PabloSwap} from "./pabloSwap.model"
import {PabloLiquidityAdded} from "./pabloLiquidityAdded.model"
import {PabloLiquidityRemoved} from "./pabloLiquidityRemoved.model"

@Entity_()
export class PabloTransaction {
    constructor(props?: Partial<PabloTransaction>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => PabloPool, {nullable: true})
    pool!: PabloPool

    @Index_()
    @Column_("text", {nullable: false})
    account!: string

    @Index_()
    @Column_("timestamp with time zone", {nullable: false})
    timestamp!: Date

    /**
     * Last updated block id
     */
    @Column_("text", {nullable: false})
    blockId!: string

    @Index_()
    @ManyToOne_(() => Event, {nullable: true})
    event!: Event | undefined | null

    @Column_("varchar", {length: 16, nullable: false})
    txType!: PabloTx

    @Index_()
    @ManyToOne_(() => PabloSwap, {nullable: true})
    swap!: PabloSwap | undefined | null

    @Index_()
    @ManyToOne_(() => PabloLiquidityAdded, {nullable: true})
    liquidityAdded!: PabloLiquidityAdded | undefined | null

    @Index_()
    @ManyToOne_(() => PabloLiquidityRemoved, {nullable: true})
    liquidityRemoved!: PabloLiquidityRemoved | undefined | null

    @Column_("bool", {nullable: false})
    success!: boolean

    @Column_("text", {nullable: true})
    failReason!: string | undefined | null
}

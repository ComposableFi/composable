import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, Index as Index_, OneToMany as OneToMany_} from "typeorm"
import * as marshal from "./marshal"
import {PabloPoolAsset} from "./pabloPoolAsset.model"
import {PabloTransaction} from "./pabloTransaction.model"

@Entity_()
export class PabloPool {
    constructor(props?: Partial<PabloPool>) {
        Object.assign(this, props)
    }

    /**
     * Pool ID
     */
    @PrimaryColumn_()
    id!: string

    /**
     * ID of the last event that was used to derive this entity data
     */
    @Column_("text", {nullable: false})
    eventId!: string

    @Index_()
    @Column_("text", {nullable: false})
    owner!: string

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    lpIssued!: bigint

    @Column_("int4", {nullable: false})
    transactionCount!: number

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    totalLiquidity!: bigint

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    totalVolume!: bigint

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    totalFees!: bigint

    @Index_()
    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    blockNumber!: bigint

    /**
     * Timestamp of the block in which this activity occurred
     */
    @Index_()
    @Column_("timestamp with time zone", {nullable: false})
    timestamp!: Date

    @OneToMany_(() => PabloPoolAsset, e => e.pool)
    poolAssets!: PabloPoolAsset[]

    @OneToMany_(() => PabloTransaction, e => e.pool)
    transactions!: PabloTransaction[]
}

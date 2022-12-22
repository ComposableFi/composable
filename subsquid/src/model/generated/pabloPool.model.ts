import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, Index as Index_, OneToMany as OneToMany_} from "typeorm"
import * as marshal from "./marshal"
import {PabloPoolType} from "./_pabloPoolType"
import {PabloPoolAsset} from "./pabloPoolAsset.model"
import {PabloAssetWeight} from "./pabloAssetWeight.model"

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

    @Column_("varchar", {length: 24, nullable: false})
    poolType!: PabloPoolType

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    lpIssued!: bigint

    @Column_("int4", {nullable: false})
    transactionCount!: number

    /**
     * Timestamp of the block in which this activity occurred
     */
    @Index_()
    @Column_("timestamp with time zone", {nullable: false})
    timestamp!: Date

    @OneToMany_(() => PabloPoolAsset, e => e.pool)
    poolAssets!: PabloPoolAsset[]

    @OneToMany_(() => PabloAssetWeight, e => e.pool)
    poolAssetWeights!: PabloAssetWeight[]
}

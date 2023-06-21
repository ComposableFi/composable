import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, Index as Index_, ManyToOne as ManyToOne_, OneToMany as OneToMany_} from "typeorm"
import {PabloPoolType} from "./_pabloPoolType"
import {PabloLpToken} from "./pabloLpToken.model"
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

    @Index_()
    @ManyToOne_(() => PabloLpToken, {nullable: true})
    lpToken!: PabloLpToken

    @Column_("int4", {nullable: false})
    transactionCount!: number

    /**
     * Timestamp of the block in which this was last updated
     */
    @Index_()
    @Column_("timestamp with time zone", {nullable: false})
    timestamp!: Date

    @OneToMany_(() => PabloPoolAsset, e => e.pool)
    poolAssets!: PabloPoolAsset[]

    @OneToMany_(() => PabloAssetWeight, e => e.pool)
    poolAssetWeights!: PabloAssetWeight[]

    /**
     * Last updated block id
     */
    @Column_("text", {nullable: false})
    blockId!: string

    /**
     * Asset to be used as reference when calculating the pool's TVL
     */
    @Column_("text", {nullable: false})
    quoteAssetId!: string
}

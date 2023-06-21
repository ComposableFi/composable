import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {PabloPool} from "./pabloPool.model"

@Entity_()
export class PabloPoolAsset {
    constructor(props?: Partial<PabloPoolAsset>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Column_("text", {nullable: false})
    assetId!: string

    @Index_()
    @ManyToOne_(() => PabloPool, {nullable: true})
    pool!: PabloPool

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    totalLiquidity!: bigint

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    totalVolume!: bigint

    /**
     * Last updated block id
     */
    @Column_("text", {nullable: false})
    blockId!: string

    @Column_("numeric", {transformer: marshal.floatTransformer, nullable: false})
    weight!: number
}

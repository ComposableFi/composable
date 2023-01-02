import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {PabloPool} from "./pabloPool.model"

@Entity_()
export class PabloAssetWeight {
    constructor(props?: Partial<PabloAssetWeight>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => PabloPool, {nullable: true})
    pool!: PabloPool

    @Index_()
    @Column_("text", {nullable: false})
    assetId!: string

    @Column_("numeric", {transformer: marshal.floatTransformer, nullable: false})
    weight!: number
}

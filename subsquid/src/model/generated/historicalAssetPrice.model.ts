import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {Currency} from "./_currency"

@Entity_()
export class HistoricalAssetPrice {
    constructor(props?: Partial<HistoricalAssetPrice>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    /**
     * ID of the asset
     */
    @Index_()
    @Column_("text", {nullable: false})
    assetId!: string

    @Column_("numeric", {transformer: marshal.floatTransformer, nullable: false})
    price!: number

    @Index_()
    @Column_("timestamp with time zone", {nullable: false})
    timestamp!: Date

    @Column_("varchar", {length: 3, nullable: false})
    currency!: Currency
}

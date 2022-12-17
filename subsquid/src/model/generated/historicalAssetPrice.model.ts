import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, Index as Index_, ManyToOne as ManyToOne_} from "typeorm"
import * as marshal from "./marshal"
import {Asset} from "./asset.model"
import {Currency} from "./_currency"

@Entity_()
export class HistoricalAssetPrice {
    constructor(props?: Partial<HistoricalAssetPrice>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    /**
     * ID of the event associated with this activity
     */
    @Index_()
    @Column_("text", {nullable: false})
    eventId!: string

    /**
     * ID of the asset
     */
    @Index_()
    @ManyToOne_(() => Asset, {nullable: true})
    asset!: Asset

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    price!: bigint

    @Index_()
    @Column_("timestamp with time zone", {nullable: false})
    timestamp!: Date

    @Column_("varchar", {length: 3, nullable: false})
    currency!: Currency
}

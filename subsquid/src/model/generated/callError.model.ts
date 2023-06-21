import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_} from "typeorm"

@Entity_()
export class CallError {
    constructor(props?: Partial<CallError>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Column_("text", {nullable: false})
    section!: string

    @Column_("text", {nullable: false})
    name!: string

    @Column_("text", {nullable: true})
    description!: string | undefined | null
}

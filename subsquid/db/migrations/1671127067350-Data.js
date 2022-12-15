module.exports = class Data1671127067350 {
    name = 'Data1671127067350'

    async up(db) {
        await db.query(`ALTER TABLE "pablo_pool" ADD "pool_type" character varying(24) NOT NULL`)
        await db.query(`ALTER TABLE "pablo_pool" ADD "transactions" jsonb`)
        await db.query(`ALTER TABLE "event" ADD "pablo_transaction" jsonb`)
    }

    async down(db) {
        await db.query(`ALTER TABLE "pablo_pool" DROP COLUMN "pool_type"`)
        await db.query(`ALTER TABLE "pablo_pool" DROP COLUMN "transactions"`)
        await db.query(`ALTER TABLE "event" DROP COLUMN "pablo_transaction"`)
    }
}

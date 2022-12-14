module.exports = class Data1670973597957 {
    name = 'Data1670973597957'

    async up(db) {
        await db.query(`ALTER TABLE "pablo_pool" DROP COLUMN "base_asset_id"`)
        await db.query(`ALTER TABLE "pablo_pool" DROP COLUMN "quote_asset_id"`)
        await db.query(`CREATE INDEX "IDX_76686140a45d0a11fadadc16f6" ON "pablo_pool" ("owner") `)
    }

    async down(db) {
        await db.query(`ALTER TABLE "pablo_pool" ADD "base_asset_id" text NOT NULL`)
        await db.query(`ALTER TABLE "pablo_pool" ADD "quote_asset_id" text NOT NULL`)
        await db.query(`DROP INDEX "public"."IDX_76686140a45d0a11fadadc16f6"`)
    }
}

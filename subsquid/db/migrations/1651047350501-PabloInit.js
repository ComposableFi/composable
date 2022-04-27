module.exports = class PabloInit1651047350501 {
  name = 'PabloInit1651047350501'

  async up(db) {
    await db.query(`CREATE TABLE "pablo_pool_asset" ("id" character varying NOT NULL, "asset_id" text NOT NULL, "total_liquidity" numeric NOT NULL, "total_volume" numeric NOT NULL, "block_number" numeric NOT NULL, "calculated_timestamp" numeric NOT NULL, "pool_id" character varying NOT NULL, CONSTRAINT "PK_fc75f8a8a8a0ac8408eef787237" PRIMARY KEY ("id"))`)
    await db.query(`CREATE INDEX "IDX_7fd4cdb45620476d1de745a265" ON "pablo_pool_asset" ("pool_id") `)
    await db.query(`CREATE TABLE "pablo_pool" ("id" character varying NOT NULL, "pool_id" text NOT NULL, "owner" text NOT NULL, "transaction_count" integer NOT NULL, "total_liquidity" text NOT NULL, "total_volume" text NOT NULL, "quote_asset_id" numeric NOT NULL, "block_number" numeric NOT NULL, "calculated_timestamp" numeric NOT NULL, CONSTRAINT "PK_28d674c3fdadf69d19745e5343a" PRIMARY KEY ("id"))`)
    await db.query(`CREATE TABLE "pablo_transaction" ("id" character varying NOT NULL, "event_id" text NOT NULL, "transaction_type" character varying(16), "base_asset_id" numeric NOT NULL, "base_asset_amount" numeric NOT NULL, "quote_asset_id" numeric NOT NULL, "quote_asset_amount" numeric NOT NULL, "block_number" numeric NOT NULL, "price_in_quote_asset" text NOT NULL, "received_timestamp" numeric NOT NULL, "pool_id" character varying NOT NULL, CONSTRAINT "PK_8b040ecc6da14a71ef547ae2ae6" PRIMARY KEY ("id"))`)
    await db.query(`CREATE INDEX "IDX_969a927080f5b6c81b79b40cd8" ON "pablo_transaction" ("pool_id") `)
    await db.query(`ALTER TABLE "pablo_pool_asset" ADD CONSTRAINT "FK_7fd4cdb45620476d1de745a2658" FOREIGN KEY ("pool_id") REFERENCES "pablo_pool"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
    await db.query(`ALTER TABLE "pablo_transaction" ADD CONSTRAINT "FK_969a927080f5b6c81b79b40cd86" FOREIGN KEY ("pool_id") REFERENCES "pablo_pool"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
  }

  async down(db) {
    await db.query(`DROP TABLE "pablo_pool_asset"`)
    await db.query(`DROP INDEX "public"."IDX_7fd4cdb45620476d1de745a265"`)
    await db.query(`DROP TABLE "pablo_pool"`)
    await db.query(`DROP TABLE "pablo_transaction"`)
    await db.query(`DROP INDEX "public"."IDX_969a927080f5b6c81b79b40cd8"`)
    await db.query(`ALTER TABLE "pablo_pool_asset" DROP CONSTRAINT "FK_7fd4cdb45620476d1de745a2658"`)
    await db.query(`ALTER TABLE "pablo_transaction" DROP CONSTRAINT "FK_969a927080f5b6c81b79b40cd86"`)
  }
}

module.exports = class Init1660638316612 {
  name = 'Init1660638316612'

  async up(db) {
    await db.query(`CREATE TABLE "historical_balance" ("id" character varying NOT NULL, "balance" numeric NOT NULL, "date" TIMESTAMP WITH TIME ZONE NOT NULL, "account_id" character varying NOT NULL, CONSTRAINT "PK_74ac29ad0bdffb6d1281a1e17e8" PRIMARY KEY ("id"))`)
    await db.query(`CREATE INDEX "IDX_383ff006e4b59db91d32cb891e" ON "historical_balance" ("account_id") `)
    await db.query(`CREATE TABLE "account" ("id" character varying NOT NULL, "balance" numeric NOT NULL, CONSTRAINT "PK_54115ee388cdb6d86bb4bf5b2ea" PRIMARY KEY ("id"))`)
    await db.query(`CREATE TABLE "pablo_pool_asset" ("id" character varying NOT NULL, "asset_id" numeric NOT NULL, "total_liquidity" numeric NOT NULL, "total_volume" numeric NOT NULL, "block_number" numeric NOT NULL, "calculated_timestamp" numeric NOT NULL, "pool_id" character varying NOT NULL, CONSTRAINT "PK_fc75f8a8a8a0ac8408eef787237" PRIMARY KEY ("id"))`)
    await db.query(`CREATE INDEX "IDX_7fd4cdb45620476d1de745a265" ON "pablo_pool_asset" ("pool_id") `)
    await db.query(`CREATE TABLE "pablo_pool" ("id" character varying NOT NULL, "event_id" text NOT NULL, "pool_id" numeric NOT NULL, "owner" text NOT NULL, "transaction_count" integer NOT NULL, "total_liquidity" text NOT NULL, "total_volume" text NOT NULL, "total_fees" text NOT NULL, "quote_asset_id" numeric NOT NULL, "block_number" numeric NOT NULL, "calculated_timestamp" numeric NOT NULL, CONSTRAINT "PK_28d674c3fdadf69d19745e5343a" PRIMARY KEY ("id"))`)
    await db.query(`CREATE TABLE "pablo_transaction" ("id" character varying NOT NULL, "event_id" text NOT NULL, "who" text NOT NULL, "transaction_type" character varying(16), "base_asset_id" numeric NOT NULL, "base_asset_amount" numeric NOT NULL, "quote_asset_id" numeric NOT NULL, "quote_asset_amount" numeric NOT NULL, "block_number" numeric NOT NULL, "spot_price" text NOT NULL, "fee" text NOT NULL, "received_timestamp" numeric NOT NULL, "pool_id" character varying NOT NULL, CONSTRAINT "PK_8b040ecc6da14a71ef547ae2ae6" PRIMARY KEY ("id"))`)
    await db.query(`CREATE INDEX "IDX_969a927080f5b6c81b79b40cd8" ON "pablo_transaction" ("pool_id") `)
    await db.query(`CREATE TABLE "bonded_finance_bond_offer" ("id" character varying NOT NULL, "event_id" text NOT NULL, "offer_id" text NOT NULL, "total_purchased" numeric NOT NULL, "beneficiary" text NOT NULL, CONSTRAINT "PK_1a7a97e3d57a4ac842dc2ef48ba" PRIMARY KEY ("id"))`)
    await db.query(`CREATE TABLE "vesting_schedule" ("id" character varying NOT NULL, "event_id" text NOT NULL, "from" text NOT NULL, "schedule_id" text NOT NULL, "to" text NOT NULL, "schedule" jsonb NOT NULL, CONSTRAINT "PK_4818b05532ed9058110ed5b5b13" PRIMARY KEY ("id"))`)
    await db.query(`ALTER TABLE "historical_balance" ADD CONSTRAINT "FK_383ff006e4b59db91d32cb891e9" FOREIGN KEY ("account_id") REFERENCES "account"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
    await db.query(`ALTER TABLE "pablo_pool_asset" ADD CONSTRAINT "FK_7fd4cdb45620476d1de745a2658" FOREIGN KEY ("pool_id") REFERENCES "pablo_pool"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
    await db.query(`ALTER TABLE "pablo_transaction" ADD CONSTRAINT "FK_969a927080f5b6c81b79b40cd86" FOREIGN KEY ("pool_id") REFERENCES "pablo_pool"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`)
  }

  async down(db) {
    await db.query(`DROP TABLE "historical_balance"`)
    await db.query(`DROP INDEX "public"."IDX_383ff006e4b59db91d32cb891e"`)
    await db.query(`DROP TABLE "account"`)
    await db.query(`DROP TABLE "pablo_pool_asset"`)
    await db.query(`DROP INDEX "public"."IDX_7fd4cdb45620476d1de745a265"`)
    await db.query(`DROP TABLE "pablo_pool"`)
    await db.query(`DROP TABLE "pablo_transaction"`)
    await db.query(`DROP INDEX "public"."IDX_969a927080f5b6c81b79b40cd8"`)
    await db.query(`DROP TABLE "bonded_finance_bond_offer"`)
    await db.query(`DROP TABLE "vesting_schedule"`)
    await db.query(`ALTER TABLE "historical_balance" DROP CONSTRAINT "FK_383ff006e4b59db91d32cb891e9"`)
    await db.query(`ALTER TABLE "pablo_pool_asset" DROP CONSTRAINT "FK_7fd4cdb45620476d1de745a2658"`)
    await db.query(`ALTER TABLE "pablo_transaction" DROP CONSTRAINT "FK_969a927080f5b6c81b79b40cd86"`)
  }
}

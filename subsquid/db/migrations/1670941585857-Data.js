module.exports = class Data1670941585857 {
  name = "Data1670941585857";

  async up(db) {
    await db.query(`ALTER TABLE "pablo_pool" DROP COLUMN "total_liquidity"`);
    await db.query(
      `ALTER TABLE "pablo_pool" ADD "total_liquidity" numeric NOT NULL`
    );
    await db.query(`ALTER TABLE "pablo_pool" DROP COLUMN "total_volume"`);
    await db.query(
      `ALTER TABLE "pablo_pool" ADD "total_volume" numeric NOT NULL`
    );
    await db.query(`ALTER TABLE "pablo_pool" DROP COLUMN "total_fees"`);
    await db.query(
      `ALTER TABLE "pablo_pool" ADD "total_fees" numeric NOT NULL`
    );
  }

  async down(db) {
    await db.query(
      `ALTER TABLE "pablo_pool" ADD "total_liquidity" text NOT NULL`
    );
    await db.query(`ALTER TABLE "pablo_pool" DROP COLUMN "total_liquidity"`);
    await db.query(`ALTER TABLE "pablo_pool" ADD "total_volume" text NOT NULL`);
    await db.query(`ALTER TABLE "pablo_pool" DROP COLUMN "total_volume"`);
    await db.query(`ALTER TABLE "pablo_pool" ADD "total_fees" text NOT NULL`);
    await db.query(`ALTER TABLE "pablo_pool" DROP COLUMN "total_fees"`);
  }
};

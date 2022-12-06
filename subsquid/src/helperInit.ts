import "dotenv/config";
import { DataSource } from "typeorm";
import fs from "fs";
import path from "path";

const sqlInitScript = fs
  .readFileSync(path.join(__dirname, "../dbInit.sql"))
  .toString();

const AppDataSource = new DataSource({
  type: "postgres",
  logging: ["error", "schema"],
  host: process.env.DB_HOST || "localhost",
  port: process.env.DB_PORT ? parseInt(process.env.DB_PORT, 10) : 5432,
  database: process.env.DB_NAME || "postgres",
  username: process.env.DB_USER || "postgres",
  password: process.env.DB_PASS || "postgres",
});

AppDataSource.initialize()
  .then(async (dataSource) => {
    try {
      await dataSource.query(sqlInitScript);
    } finally {
      await dataSource.destroy().catch(() => null);
    }
  })
  .then(
    () => {
      console.log("Helper functions successfully created.");
      process.exit(0);
    },
    (err) => {
      console.error(err);
      process.exit(1);
    }
  )
  .catch((err) => {
    console.error("Error during helper functions initialization", err);
    process.exit(1);
  });

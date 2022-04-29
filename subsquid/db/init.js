require('dotenv/config')

const ormconfig = {
  type: 'postgres',
  // entities: [require.resolve('../lib/model')],
  entities: [],
  migrations: [__dirname + '/migrations/*.js'],
  synchronize: false,
  migrationsRun: false,
  dropSchema: false,
  logging: ["query", "error", "schema"],
  host: process.env.DB_HOST || 'localhost',
  port: process.env.DB_PORT ? parseInt(process.env.DB_PORT) : 5432,
  database: process.env.DB_NAME || 'postgres',
  username: process.env.DB_USER || 'postgres',
  password: process.env.DB_PASS || 'postgres'
}

require('typeorm').createConnection(ormconfig).then(async con => {
  try {
    await con.runMigrations({transaction: 'all'})
  } finally {
    await con.close().catch(err => null)
  }
}).then(
  () => process.exit(),
  err => {
    console.error(err)
    process.exit(1)
  }
)

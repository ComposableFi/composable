database:
"postgres://${database.user}:${database.password}@${database.host}:${
  toString database.port
}/${database.name}"

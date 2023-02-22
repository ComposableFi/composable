#!/bin/sh

set -eou pipefail

# main entry point for the script
main() {
  #  ${foo:="default value if unset"}
  if ${TRIGGER_REPROCESSING:=false} ; then
    run_steps_to_trigger_reprocessing
    export TRIGGER_REPROCESSING=false
  fi

  log INFO "Running db migration"
  npx squid-typeorm-migration apply || log DIE "DB migration step failed"

  log INFO "Running sqlinit "
  node lib/helperInit.js || log DIE  "Sqlinit step failed"

  log INFO "Starting the processor" 
  node -r dotenv/config lib/processor.js || log DIE "Starting the processor step failed"
}

run_steps_to_trigger_reprocessing() {
  log INFO "Reprocessing triggered"
  set_db_connection_details
  psql -c "DROP SCHEMA public CASCADE ;
           DROP SCHEMA squid_processor cascade ;
           create SCHEMA public  ;
           create schema squid_processor ;"
}

set_db_connection_details() {
  # https://www.postgresql.org/docs/current/libpq-envars.html
  export PGHOST="${DB_HOST}"
  export PGDATABASE="${DB_NAME}"
  export PGPASSWORD="${DB_PASS}"
  export PGUSER="${DB_USER}"
}


log() {
  lvl=$1 msg=$2
  fmt='+%Y-%m-%d %H:%M:%S'
  lg_date=$(date "${fmt}")
  if [[ "${lvl}" = "DIE" ]] ; then
    lvl="ERROR"
   echo "${lg_date} - ${lvl} - ${msg}" 
   exit 1
  else
    echo "${lg_date} - ${lvl} - ${msg}" 
  fi
}

main

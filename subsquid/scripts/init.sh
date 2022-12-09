#!/bin/sh


function log {
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


log INFO "Running db migration"
npx squid-typeorm-migration apply || log DIE "DB migration step failed"


log INFO "Running sqlinit "
node lib/helperInit.js || log DIE  "Sqlinit step failed"

log INFO "Starting the processor" 
node -r dotenv/config lib/processor.js || log DIE "Starting the processor step failed"



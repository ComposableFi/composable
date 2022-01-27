import minimist from "minimist";

/**
 * Start program using following parameters:
 * Host: -h 127.0.0.1
 * Port: -p 9988
 */
export const args = minimist(process.argv.slice(2),
  { default: { p: 9988, h: '127.0.0.1', w: true  } });

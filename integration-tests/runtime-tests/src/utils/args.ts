import minimist from "minimist";


export const args = minimist(process.argv.slice(2), 
  { default: { p: 9988, h: '127.0.0.1', w: true  } });
console.debug(`args parser: Host(${args.h}), Port(${args.p})`);

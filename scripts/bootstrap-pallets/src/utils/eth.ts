export function privateKeyFromSeed(seed: number): string {
  return "0x" + seed.toString(16).padStart(64, "0");
}

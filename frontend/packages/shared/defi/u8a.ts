export function concatU8a(a: Uint8Array, b: Uint8Array): Uint8Array {
  const c = new Uint8Array(a.length + b.length);
  c.set(a);
  c.set(b, a.length);
  return c;
}

export function compareU8a(a: Uint8Array, b: Uint8Array): boolean {
  if (a.length !== b.length) return false;

  let equal = true;

  a.forEach((a, i) => {
    if (a != b[i]) {
      equal = false;
    }
  });

  return equal;
}

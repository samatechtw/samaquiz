// Return a string representation of `n` with minimum 2 digits
export function fixed2(n: number): string {
  return n < 10 ? `0${n}`.slice(-2) : `${n}`
}

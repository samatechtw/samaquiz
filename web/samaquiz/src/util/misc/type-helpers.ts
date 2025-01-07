// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const omit = <T extends Record<string, any>, K extends keyof T>(
  target: T,
  key: K,
): Omit<T, K> => {
  const obj = { ...target }
  delete obj[key]
  return obj
}

export const sortById = <T extends { id: string }>(
  items: T[],
  idOrder: string[],
): T[] => {
  const lookup: Record<string, T> = {}
  for (const item of items) {
    lookup[item.id] = item
  }
  return idOrder.map((id) => lookup[id]).filter((obj): obj is T => obj !== undefined)
}

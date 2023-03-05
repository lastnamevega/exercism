type Nested<T> = T | Nested<T>[];

export function accumulate<T>(list: T[], accumulator: (element: T) => Nested<T>) {
  return list.map(element => accumulator(element))
}

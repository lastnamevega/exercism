import * as _ from 'lodash';

type Numbers = (number | Numbers | undefined)[];

export function flatten(numbers: Numbers): number[] {
  return _.flattenDeep(numbers).filter(
    (element) => typeof element === 'number'
  ) as number[];
}

export class Triangle {
  constructor(...sides) {
    this._sides = sides.sort();
    const unequal = this._sides[0] + this._sides[1] >= this._sides[2];
    this._valid = unequal && this._sides[2] > 0;
    this._isosceles = sides[0] === sides[1] || sides[1] === sides[2];
    this._equilateral = this._sides[0] === this._sides[2];
  }

  get isEquilateral() {
    return this._valid && this._equilateral;
  }

  get isIsosceles() {
    return this._valid && this._isosceles;
  }

  get isScalene() {
    return this._valid && !this._isosceles;
  }
}

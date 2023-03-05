enum ColorValue {
  black,
  brown,
  red,
  orange,
  yellow,
  green,
  blue,
  violet,
  grey,
  white,
}

type Color = keyof typeof ColorValue;

export class ResistorColor {
  private colors: Color[];

  constructor(colors: Color[]) {
    if (colors.length < 2) {
      throw "At least two colors need to be present";
    }

    this.colors = colors.slice(0, 2);
  }

  value(): number {
    const firstDigit = 10 * ColorValue[this.colors[0]];
    const secondDigit = ColorValue[this.colors[1]];
    return firstDigit + secondDigit;
  }
}

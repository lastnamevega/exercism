export const transform = (old) => {
  let transformed = {};

  Object.entries(old).map(([s, letters]) => {
    const score = parseInt(s);

    letters.map((letter) => {
      transformed[letter.toLowerCase()] = score;
    });
  });

  return transformed;
};

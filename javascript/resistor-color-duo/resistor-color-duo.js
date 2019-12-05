//
// This is only a SKELETON file for the 'Resistor Color Duo' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const value = (colors) => {
  const [first, second] = colors;
  return COLORS.indexOf(first) * 10 + COLORS.indexOf(second);
};

export const COLORS = ["black","brown","red","orange","yellow","green","blue","violet","grey","white"];

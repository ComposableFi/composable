export const stringToNumber = (value: string) => {
  try {
    return Number(value.replaceAll(",", ""));
  } catch {
    return Number(value);
  }
};

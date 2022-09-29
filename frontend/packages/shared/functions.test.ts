import { callbackGate } from "./functions";

describe("functions.ts", () => {
  it("gate", () => {
    const callback = jest.fn((...a) =>
      a.reduce((acc, cur) => {
        acc += cur;
        return acc;
      }, 0)
    );

    callbackGate(callback, 1, 2, 3);

    expect(callback).toHaveBeenCalledTimes(1);
    expect(callback).toHaveReturnedWith(6);
  });

  it("callbackGate() false values should not call the callback", () => {
    const callback = jest.fn((...a) =>
      a.reduce((acc, cur) => {
        acc += cur;
        return acc;
      }, 0)
    );
    callbackGate(callback, 1, undefined, 3);

    expect(callback).toHaveBeenCalledTimes(0);

    callbackGate(callback);

    expect(callback).toHaveBeenCalledTimes(0);
  });
});

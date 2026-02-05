export const fakeDelay = (ms: number) =>
  new Promise<void>((resolve, _) => {
    setTimeout(() => resolve(), ms);
  });

export const fakeDelayFailure = (ms: number) =>
  new Promise<void>((resolve, reject) => {
    setTimeout(() => {
      // ~33% chance to fail
      if (Math.random() < 1 / 3) {
        reject(new Error("fakeDelay failed intentionally (33% chance)"));
        return;
      }
      resolve();
    }, ms);
  });

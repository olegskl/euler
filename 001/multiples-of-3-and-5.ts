// Find the sum of all the multiples of 3 or 5 below 1000.

// O(n)
function naiveSolution(): number {
  let sum = 0;
  for (let n = 0; n < 1000; n += 1) {
    if (n % 3 === 0 || n % 5 === 0) {
      sum += n;
    }
  }
  return sum;
}

if (naiveSolution() !== 233168) {
  throw new Error("Naive solution is incorrect.");
}

function arithmeticSeries(divisor: number): number {
  let n = Math.floor(999 / divisor);
  return divisor * n * (n + 1) / 2;
}

// O(1)
function optimalSolution(): number {
  return arithmeticSeries(3) + arithmeticSeries(5) - arithmeticSeries(3 * 5);
}

if (optimalSolution() !== 233168) {
  throw new Error("Optimal solution is incorrect.");
}

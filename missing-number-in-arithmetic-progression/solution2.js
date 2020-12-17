/**
 * @param {number[]} arr
 * @return {number}
 */
var missingNumber = function(arr) {
  const n = arr.length;

  const s1 = ((n + 1) * (arr[0] + arr[arr.length - 1])) / 2;
  const s2 = arr.reduce((acc, v) => acc + v, 0);
  return s1 - s2;
};

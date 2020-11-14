/**
 * @param {number} buckets
 * @param {number} minutesToDie
 * @param {number} minutesToTest
 * @return {number}
 */
var poorPigs = function(buckets, minutesToDie, minutesToTest) {
  var t = minutesToTest / minutesToDie;
  var x = 0;
  while (Math.pow(t+1, x) < buckets) {
    x++;
  }
  return x;
};

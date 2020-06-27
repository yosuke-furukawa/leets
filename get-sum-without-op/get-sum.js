/**
 * @param {number} a
 * @param {number} b
 * @return {number}
 */
var getSum = function(a, b) {
  var common = a & b;
  var uncommon = a ^ b;
  while (common !== 0) {
    common = a & b;
    uncommon = a ^ b;
    a = uncommon;
    b = common << 1;
  }
  return uncommon;
};

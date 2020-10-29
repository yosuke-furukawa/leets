/**
 * @param {number} n
 * @return {number}
 */
var findNthDigit = function(n) {
  var len = 1;
  var cnt = 9;
  var start = 1;
  while (n > len * cnt) {
    n = n - len * cnt;
    cnt = cnt * 10;
    start = start * 10;
    len++;
  }
  start += parseInt((n-1)/len);
  var s = `${start}`;
  return +s[(n-1)%len];
};

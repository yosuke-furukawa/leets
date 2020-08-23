/**
 * @param {number} N
 * @return {number}
 */
var map = new Map();
map.set(0, 0);
map.set(1, 1);
map.set(2, 1);
map.set(3, 2);
map.set(4, 3);

var fib = function(N) {
  if (map.has(N)) {
    return map.get(N);
  }
  var ans = fib(N-1) + fib(N-2);
  map.set(N, ans);
  return ans;
};

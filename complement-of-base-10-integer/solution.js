/**
 * @param {number} N
 * @return {number}
 */
var bitwiseComplement = function(N) {
  if (N === 0) {
    return 1;
  }
  var v = N;
  v = v | (v >> 1);
  v = v | (v >> 2);
  v = v | (v >> 4);
  v = v | (v >> 8);
  v = v | (v >> 16);
  v = v ^ (v >> 1);
  
  if (v < N) {
    v *= 2;
  }
  return (~N & (v-1))
};

/**
 * @param {number[]} A
 * @return {boolean}
 */
var validMountainArray = function(A) {
  var peek = 0;
  if (A.length < 3) {
    return false;
  }
  var prev;
  var curr;
  var next;
  var climbing = true;
  for (var i=1;i<A.length-1;i++) {
    prev = A[i-1];
    curr = A[i];
    next = A[i+1] ?? -1;
    if (climbing) {
      if (prev < curr && curr < next) {
        continue;
      } else if (prev < curr && curr > next) {
        climbing = false;
        continue;
      } else {
        return false;
      }
    } else {
      if (prev > curr && curr > next) {
        continue;
      } else {
        return false;
      }
    }
  }
  return !climbing;
};

/**
 * @param {string} A
 * @param {string} B
 * @return {boolean}
 */
var buddyStrings = function(A, B) {
  var amap = new Map();
  var dup = false;
  var switched = false;
  var i = -1;
  var j = -1;
  if (A.length !== B.length) {
    return false;
  }
  for (var ai=0;ai<A.length;ai++) {
    if (A[ai] !== B[ai]) {
      if (j >= 0) {
        return false;
      } else if (i >= 0) {
        j = ai;
        if (A[i] !== B[j] || A[j] !== B[i]) {
          return false;
        } else {
          switched = true;
        }
      } else {
        i = ai;
      }
    }
    if (amap.has(A[ai])) {
      amap.get(A[ai]).push(ai);
      dup = true;
      continue;
    }
    amap.set(A[ai], [ai]);
  }
  if (switched) {
    return true;
  } else if (i>=0) {
    return false;
  } else if (dup) {
    return true;
  } else {
    return false;
  }
  
};

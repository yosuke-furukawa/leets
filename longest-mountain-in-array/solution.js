/**
 * @param {number[]} A
 * @return {number}
 */
var longestMountain = function(A) {
  var max = 0;
  var length = 0;
  var state = "flat";
  for (var i=0;i<A.length;i++) {
    var m1 = A[i];
    var m2 = A[i+1] ?? -10000000;
    var m3 = A[i+2] ?? -Infinity;
    // console.log(state, m1, m2, m3);
    if (state === "flat") {
      if (m1 < m2 && m2 < m3) {
        length++;
        state = "up";
      } else if (m1 < m2 && m2 > m3) {
        length++;
        state = "peek";
      } else {
        length = 0;
        state = "flat";
      }
    } else if (state === "up") {
      if (m1 < m2 && m2 < m3) {
        length++;
        state = "up";
      } else if (m1 < m2 && m2 > m3) {
        length++;
        state = "peek";
      } else {
        length = 0;
        state = "flat";
      }
    } else if (state === "peek") {
      if (m1 > m2) {
        length++;
        state = "down";
      } else {
        length = 0;
        state = "flat";
      }
    } else if (state === "down") {
      if (m1 > m2) {
        length++;
        state = "down";
      } else {
        length++;
        if (m1 < m2) {
          state = "up";
          i--;
          max = Math.max(max, length);
          length = 0;
          continue;
        } else if (m1 === m2) {
          state = "flat";
          max = Math.max(max, length);
          length = 0;
          continue;
        }
      }
      max = Math.max(max, length);
    }
  }
  return max;
};

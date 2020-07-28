var oddJumps = function(A, i) {
  var min = Infinity;
  var minIndex = i;
  for (var j=i+1;j<A.length;j++) {
    if (A[i] <= A[j]) {
      if (min > A[j]) {
        min = A[j];
        minIndex = j;
      }
    }
  }
  return minIndex;
}

var evenJumps = function(A, i) {
  var max = -1;
  var maxIndex = i;
  for (var j=i+1;j<A.length;j++) {
    if (A[i] >= A[j]) {
      if (max < A[j]) {
        max = A[j];
        maxIndex = j;
      }
    }
  }
  return maxIndex;
}

/**
 * @param {number[]} A
 * @return {number}
 */
var oddEvenJumps = function(A) {
  var count = 1;
  var map = new Map();
  for (var i=A.length-2;i>=0;i--) {
    var jumps = 1;
    var n = i;
    while (true) {
      if (n === A.length-1) {
        map.set(`${n}, ${jumps%2}`, "reached");
        count++;
        break;
      }
      if (map.get(`${n}, ${jumps%2}`) === "reached") {
        count++;
        map.set(`${i}, 1`, "reached");
        break;
      }
      var next;
      if (jumps % 2 === 0) {
        next = evenJumps(A, n);
        jumps++;
      } else {
        next = oddJumps(A, n);
        jumps++;
      }
      if (n === next) {
        break;
      }

      n = next;
    }
  }
  return count;
};

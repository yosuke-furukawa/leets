/**
 * @param {number[]} A
 * @return {number[]}
 */
var flip = (A, i) => {
  return [...A.slice(0, i+1).reverse(), ...A.slice(i+1)];
}

var pancakeSort = function(A) {
  var results = [];
  var i = A.length-1;
  while(i > 0) {
    if (A[i] === i+1) {
      i--;
      continue;
    }
    var j;
    for (j=0;j<i;j++) {
      if (A[j] === i+1) {
        break;
      }
    }

    results.push(j+1);
    A = flip(A, j);
    results.push(i+1);
    A = flip(A, i);
    i--;
  }
  return results;
};

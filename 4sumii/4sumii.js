/**
 * @param {number[]} A
 * @param {number[]} B
 * @param {number[]} C
 * @param {number[]} D
 * @return {number}
 */
var fourSumCount = function(A, B, C, D) {
  // A + B === -C + -D
  var abmap = new Map();
  var result = 0;
  for (var a of A) {
    for (var b of B) {
      var k = a + b;
      var count = abmap.get(k) || 0;
      count++;
      
      abmap.set(k, count);
    }
  }
  console.log(abmap);
  for (var c of C) {
    for (var d of D) {
      var k = -c + -d;
      var value = abmap.get(k);
      if (value > 0) {
        result += value;
      }
    }
  }
  
  return result;
};

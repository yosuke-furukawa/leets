/**
 * @param {number} n
 * @return {number}
 */
var countPrimes = function(n) {
  var count = 0;
  var isPrimes = [false, false];
  for (var i=2;i<n;i++) {
      isPrimes[i] = true;
  }
  for (var i=2;i*i <n; i++){
      if (!isPrimes[i]) {
          continue;
      }
      for (var j = i * i; j < n; j += i) {
         isPrimes[j] = false;
      }
  }
  for (var j=0;j<isPrimes.length;j++) {
      if (isPrimes[j]) {
          count++;
      }
  }
  return count;
};

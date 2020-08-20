/**
 * @param {number} N
 * @return {boolean}
 */
var divisorGame = function(N) {
  if (N === 1) {
    return false;
  }
  if (N % 2===0) {
    return true;
  }
  
  return !prime.includes(N);
};

var prime = [];

var primes = function(N) {
  var nums = [];
  for (var i=2;i<=N;i++) {
    nums.push(i);
  }
  
  
  while (nums.length != 0) {
    var n1 = nums.pop();
    nums = nums.filter((n2) => n2 % n1 !== 0);
    prime.push(n1);
  }
}

primes(1000);

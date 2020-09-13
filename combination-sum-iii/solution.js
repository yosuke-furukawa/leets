/**
 * @param {number} k
 * @param {number} n
 * @return {number[][]}
 */

var combinationSum3 = function(k, n) {
  var combination = [];
  var backtrack = (candidate, sum) => {
    if (candidate.length === k && sum === n) {
      combination.push([...candidate]);
      return;
    }
    
    var latest = candidate[candidate.length-1] ?? 0;
    for (var i=latest+1;i<10;i++) {
      if (sum + i <= n && candidate.length+1<=k) {
        candidate.push(i);
        backtrack(candidate, sum+i);
        candidate.pop();
      }
    }
  };
  backtrack([], 0);
  return combination;
};

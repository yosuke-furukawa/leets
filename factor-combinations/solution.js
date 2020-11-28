/**
 * @param {number} n
 * @return {number[][]}
 */
var getFactors = function(n) {
  if (n === 1) {
    return [];
  }
  var results = [];
  var backtrack = (candidates, factor = 0) => {
    if (factor === n) {
      results.push([...candidates]);
      return;
    }
    
    for (var i=candidates[candidates.length-1] ?? 2;i<=n/2;i++) {
      var newCandidates = [...candidates];
      if (n % i === 0) {
        newCandidates.push(i);
        var factor = newCandidates.reduce((acc, curr) => acc*curr, 1);
        if (factor <= n) {
          backtrack(newCandidates, factor);
        } else if (factor > n) {
          break;
        }
        newCandidates.pop();
      }
    }
  }
  backtrack([]);
  return results;
};

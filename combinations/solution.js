/**
 * @param {number} n
 * @param {number} k
 * @return {number[][]}
 */
var combine = function(n, k) {
  var result = [];
  var backtrack = (first, nums) => {
    if (nums.length === k) {
      result.push([...nums]);
    }
    
    for (var i=first; i<n+1;i++) {
      nums.push(i);
      backtrack(i+1, nums);
      nums.pop();
    } 
  };
  
  backtrack(1, []);
  return result;
};

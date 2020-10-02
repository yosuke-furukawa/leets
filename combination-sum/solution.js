/**
 * @param {number[]} candidates
 * @param {number} target
 * @return {number[][]}
 */
var combinationSum = function(candidates, target) {
  var result = [];
  var backtrack = (list, position = 0) => {
    if (list.length > 0 && list.reduce((acc, curr) => acc + curr) === target) {
      result.push(list);
      return;
    }
    
    for (var i=position;i<candidates.length;i++) {
      if (list.length === 0 || list.reduce((acc, curr) => acc + curr) + candidates[i] <= target) {
        list.push(candidates[i]);
        backtrack([...list], i);
        list.pop();      
      }
    }
  };
  backtrack([]);
  
  return result;
};

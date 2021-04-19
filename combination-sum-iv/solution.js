/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */

function permutation(n, r) {
  for(var i = 0, res = 1; i < r; i++) {
    res *= n - i;
  }
  return res;
}

function combination(n, r) {
  return permutation(n, r) / permutation(r, r);
}

var combinationSum4 = function(nums, target) {
  var result = 0;
  var backtrack = (list, pos = 0) => {
    if (list.length > 0 && list.reduce((acc, curr) => acc + curr) === target) {
      console.log(list);
      var set = new Set(list);
      var len = list.length;
      var comp = 1;
      if (set.size === 1) {
        result += 1;
        return;
      }
      for (const item of set) {
        var count = 0;
        for (const l of list) {
          if (l == item) {
            count++;
          }
        }
        comp *= combination(len, count);
        len -= count;
      }
      console.log(comp);
      result += comp;
      return;
    }
    
    for (var i=pos;i<nums.length;i++) {
      if (list.length === 0 || list.reduce((acc, curr) => acc + curr) + nums[i] <= target) {
        list.push(nums[i]);
        backtrack([...list], i);
        list.pop();      
      }
    }
  };
  backtrack([]);
  
  console.log(result);
  return result;

};

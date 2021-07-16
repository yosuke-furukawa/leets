/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[][]}
 */
var fourSum = function(nums, target) {
  var sorted = nums.sort((a, b) => a-b);
  var set = new Set();
  var results = [];
  var start1 = 0;
  var end1 = sorted.length - 1;
  for (var start1=0;start1<sorted.length;start1++) {
    for (var end1=sorted.length-1;end1>start1;end1--) {
      var a = sorted[start1];
      var b = sorted[end1];
      var start2 = start1+1;
      var end2 = end1 - 1;
      while (start2 < end2) {
        var c = sorted[start2];
        var d = sorted[end2];
        var sum = a + b + c + d;
        // console.log({a, b, c, d, sum});
        if(sum > target) {
          end2--;
        } else if(sum < target) {
          start2++
        } else {
          var answer = [a,c,d,b];
          if (!set.has(answer.toString())) {
            results.push(answer);
            set.add(answer.toString());
          }
          if (c + d > 0) {
            end2--;      
          } else {
            start2++;
          }
        }
      }
    }
  }
  return results;
};

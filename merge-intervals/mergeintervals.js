/**
 * @param {number[][]} intervals
 * @return {number[][]}
 */
var merge = function(intervals) {
  if (intervals.length === 0) return intervals;
  intervals = intervals.sort((a, b) => (a[0] - b[0]));
  var merged = [];
  var head = intervals[0];
  var index = 0;
  for (var index=0;index < intervals.length;index++) {
    if (merged.length === 0 || merged[merged.length - 1][1] < intervals[index][0]) {
      merged.push(intervals[index]);
    } else {
      
      merged[merged.length-1][1] = Math.max(merged[merged.length -1][1], intervals[index][1]);
    }
  }
  return merged;
};

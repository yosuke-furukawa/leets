/**
 * @param {number[][]} intervals
 * @param {number[]} newInterval
 * @return {number[][]}
 */
var insert = function(intervals, newInterval) {
  var result = [];
  var duration = [...newInterval];
  for (var interval of intervals) {
    var [start, end] = interval;
    var [newstart, newend] = duration;
    
    if (start <= newend && newstart <= end) {
      duration = [Math.min(start, newstart), Math.max(end, newend)];
    } else if (start > newstart && end < newend) {
      continue;
    } else {
      result.push([start, end]);
    }
  }
  result.push(duration);
  result.sort((a,b) => a[0] - b[0]);
  return result;
};

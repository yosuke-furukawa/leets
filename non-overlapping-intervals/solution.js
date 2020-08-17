/**
 * @param {number[][]} intervals
 * @return {number}
 */
var eraseOverlapIntervals = function(intervals) {
  intervals = intervals.sort((a, b) => a[0] == b[0] ? a[1] - b[1] : a[0] - b[0]);
  var result = 0;
  var next = Infinity;
  for (var i=intervals.length-1;i>=0;i--) {
    if(next < intervals[i][1]) {
      result++;
    } else {
      next = intervals[i][0];
    }
  }
  return result;
};

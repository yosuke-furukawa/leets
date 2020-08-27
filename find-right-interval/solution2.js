/**
 * @param {number[][]} intervals
 * @return {number[]}
 */
var findRightInterval = function(intervals) {
  var endIntervals = [...intervals];
  var hash = new Map();
  for (var i=0;i<intervals.length;i++) {
    hash.set(intervals[i], i);
  }
  intervals.sort((a, b) => a[0] - b[0]);
  endIntervals.sort((a, b) => a[1] - b[1]);
  var j = 0;
  var result = new Array(intervals.length);
  for (var i=0;i<endIntervals.length;i++) {
    while (j < intervals.length && intervals[j][0] < endIntervals[i][1]) {
      j++;
    }
    result[hash.get(endIntervals[i])] = j === intervals.length ? -1 : hash.get(intervals[j]);
  }
  return result;
};

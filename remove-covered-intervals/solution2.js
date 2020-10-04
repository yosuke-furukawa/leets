/**
 * @param {number[][]} intervals
 * @return {number}
 */
var removeCoveredIntervals = function(intervals) {
  var sorted = intervals.sort((a, b) => {
    return a[0] == b[0] ? b[1] - a[1]: a[0] - b[0];
  });
  var count = 0;
  var end = 0;
  var prevEnd = 0;
  for (var curr of sorted) {
    end = curr[1];
    if (prevEnd < end) {
      ++count;
      prevEnd = end;
    }
  }
  return count;
};

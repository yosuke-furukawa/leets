/**
 * @param {number[][]} intervals
 * @return {number[]}
 */
var findRightInterval = function(intervals) {
  var result = [];
  for (var i = 0;i<intervals.length;i++) {
    var interval1 = intervals[i];
    var min = Infinity;
    var minPos = -1;
    for (var j=0;j<intervals.length;j++) {
      if (i === j) {
        continue;
      }
      var interval2 = intervals[j];
      if (interval1[1] <= interval2[0]) {
        var diff = interval2[0] - interval1[1];
        if (min > diff) {
          min = diff;
          minPos = j;
        }
      }
    }
    result.push(minPos);
  }
  return result;
};

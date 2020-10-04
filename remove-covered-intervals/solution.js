/**
 * @param {number[][]} intervals
 * @return {number}
 */
var removeCoveredIntervals = function(intervals) {
  var sorted = intervals.sort((a, b) => (b[1] - b[0]) - (a[1] - a[0]));
  var length = sorted.length;
  for (var i=0;i<sorted.length;i++) {
    var interval1 = sorted[i];
    for (var j=i+1;j<sorted.length;j++) {
      var interval2 = sorted[j];
      // console.log({interval1, interval2});
      if (interval1[0] <= interval2[0] && interval1[1] >= interval2[1]) {
        sorted.splice(j, 1);
        j--;
      }
    }
  }
  return sorted.length;
};

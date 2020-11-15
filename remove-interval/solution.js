/**
 * @param {number[][]} intervals
 * @param {number[]} toBeRemoved
 * @return {number[][]}
 */
var removeInterval = function(intervals, toBeRemoved) {
  var results = [];
  for (var i=0;i<intervals.length;i++) {
    var interval = intervals[i];
    if (interval[0] < toBeRemoved[1] && interval[1] > toBeRemoved[0]) {
      // interval ∈ toBeRemoved
      if (toBeRemoved[0] < interval[0] && toBeRemoved[1] > interval[1]) {
        continue;
      } else if (toBeRemoved[0] > interval[0] && toBeRemoved[1] < interval[1]) { // interval ∋ toBeRemoved
        results.push([interval[0], toBeRemoved[0]]);
        results.push([toBeRemoved[1], interval[1]]);
      } else if (interval[0] < toBeRemoved[0]) {
        results.push([interval[0], toBeRemoved[0]]);
      } else if (interval[1] > toBeRemoved[1]) {
        results.push([toBeRemoved[1], interval[1]]);
      }
    } else {
      results.push(interval);
    }
  }
  return results;
};

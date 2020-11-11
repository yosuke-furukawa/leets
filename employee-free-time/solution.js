/**
 * // Definition for an Interval.
 * function Interval(start, end) {
 *    this.start = start;
 *    this.end = end;
 * };
 */

/**
 * @param {Interval[][]} schedule
 * @return {Interval[]}
 */
var employeeFreeTime = function(schedule) {
  var sorted = schedule.flat().sort((a, b) => { 
    var startDiff = a.start - b.start;
    var endDiff = a.end - b.end;
    if (startDiff === 0) {
      return endDiff;
    }
    return startDiff;
  });

  var current = sorted.shift().end;
  var result = [];
  for (var i=0;i<sorted.length;i++) {
    var item = sorted[i];
    if (item.start > current) {
      result.push({start: current, end:item.start});
    }
    if (item.end > current) {
      current = item.end;
    }
  }
  return result
};

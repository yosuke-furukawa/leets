/**
 * @param {number[][]} intervals
 * @return {boolean}
 */
var canAttendMeetings = function(intervals) {
  var sorted = intervals.sort((a, b) => {
    var startDiff = a[0] - b[0];
    return startDiff;
  });
  
  for (var i=0;i<sorted.length-1;i++) {
    var meeting1 = sorted[i]; 
    var meeting2 = sorted[i+1];
    if (meeting1[0] < meeting2[1] && meeting1[1] > meeting2[0]) {
      return false;
    }
  }
  return true;
};

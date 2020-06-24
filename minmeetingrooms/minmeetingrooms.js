/**
 * @param {number[][]} intervals
 * @return {number}
 */
var minMeetingRooms = function(intervals) {
  var count = 0;
  intervals = intervals.sort((a, b) => a[0] === b[0] ? b[1] - a[1] : a[0] - b[0]);
  var meetings = [];
  for (const interval of intervals) {
    if (meetings.length === 0) {
      meetings.push(interval);
      continue;
    }
    const latestMeeting = meetings[meetings.length-1];
    if (interval[0] < latestMeeting[1] && interval[1] > latestMeeting[0]) {
      meetings.push(interval);
    } else {
      meetings[meetings.length-1][1] = Math.max(interval[1], latestMeeting[1]);
    }
    meetings.sort((a, b) => b[1] - a[1]);
  }
    
  return meetings.length;
};

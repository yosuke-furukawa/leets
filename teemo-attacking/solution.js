/**
 * @param {number[]} timeSeries
 * @param {number} duration
 * @return {number}
 */
var findPoisonedDuration = function(timeSeries, duration) {
  if (timeSeries.length === 0) {
    return 0;
  }
  var time = 0;
  for (var i=0;i<timeSeries.length-1;i++) {
    time += Math.min(timeSeries[i+1] - timeSeries[i], duration);
  }
  
  return time + duration;
};

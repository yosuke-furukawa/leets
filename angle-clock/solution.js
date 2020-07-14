/**
 * @param {number} hour
 * @param {number} minutes
 * @return {number}
 */
var angleClock = function(hour, minutes) {
  var hourangle = (hour * 30 + minutes * 0.5);
  var minutesangle = minutes * 6;
  return Math.min(360 - Math.abs(minutesangle - hourangle), Math.abs(minutesangle - hourangle));
};

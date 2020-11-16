/**
 * @param {number[][]} flights
 * @param {number[][]} days
 * @return {number}
 */
var maxVacationDays = function(flights, days) {
  if (days.length == 0 || flights.length == 0) return 0;
  var dp = new Array(days.length).fill(0);
  for (var week = days[0].length-1;week>=0;week--) {
    var temp = new Array(days.length).fill(0);
    for (var current=0;current<days.length;current++) {
      temp[current] = days[current][week] + dp[current];
      for (var destination=0;destination<days.length;destination++) {
        if (flights[current][destination] === 1) {
          temp[current] = Math.max(days[destination][week] + dp[destination], temp[current]);
        }
      }
    }
    dp = temp;
  }
  
  return dp[0];
};

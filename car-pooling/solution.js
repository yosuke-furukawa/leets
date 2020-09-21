/**
 * @param {number[][]} trips
 * @param {number} capacity
 * @return {boolean}
 */
var carPooling = function(trips, capacity) {
  var pickingups = [];
  for (var i=0;i<trips.length;i++) {
    var trip = trips[i];
    if (pickingups[trip[1]] != null) {
      pickingups[trip[1]] += trip[0];      
    } else {
      pickingups[trip[1]] = trip[0];      
    }
    if (pickingups[trip[2]] != null) {
      pickingups[trip[2]] -= trip[0];      
    } else {
      pickingups[trip[2]] = -trip[0];      
    } 
  }
  
  pickingups = pickingups.filter(Boolean);
  // console.log(pickingups);
  var seats = 0;
  for (const p of pickingups) {
    seats += p;
    if (seats > capacity) {
      return false;
    }
  }
  return true;
};

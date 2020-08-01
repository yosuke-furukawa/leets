/**
 * @param {number[]} seats
 * @return {number}
 */
var maxDistToClosest = function(seats) {
  var maxlen = 0;
  var maxpos = 0;
  for (var i=0;i<seats.length;i++) {
    var s1 = i === -1 ? 1 : seats[i];
    if (s1 === 0) {
      continue;
    }
    let pos = i;
    let len = 0;
    for (var j=i+1;j<seats.length;j++) {
      var s2 = seats[j];
      if (s2 === 0) {
        len++;
      } else {
        break;
      }
    }
    i = j-1;
    if (maxlen < len) {
      maxlen = len;
      maxpos = pos;
    }
  }
  
  let flen = 0;
  for (var i=0;i<seats.length;i++) {
    if (seats[i] === 0) {
      flen++;
    } else {
      break;
    }
  }
  
  let llen = 0;
  for (var i=seats.length-1;i>=0;i--) {
    if (seats[i] === 0) {
      llen++;
    } else {
      break;
    }
  }
  
  var result = Math.ceil(maxlen / 2);
  return Math.max(flen, llen, result);
};

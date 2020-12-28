/**
 * @param {number} target
 * @return {number}
 */
var reachNumber = function(target) {
  var t = Math.abs(target);
  var n = 0;
  for (n=0;n<10**9;n++) {
    if (t <= (n**2+n)/ 2) {
      break;
    }
  }
  var max = ((n**2+n)/2);
  if (max % 2 === t % 2) {
    return n;
  }
  
  if (t % 2 === 1) {
    if (n % 4 === 0) {
      return n+1;
    } else if (n % 4 === 3){
      return n+2;
    }
  } else {
    if (n % 4 === 1) {
      return n+2;
    } else if (n % 4 === 2) {
      return n+1;
    }
  }
};

/**
 * @param {number[]} position
 * @param {number} m
 * @return {number}
 */
var maxDistance = function(position, b) {
  position = position.sort((a, b) => a-b);
  var l = 0, h = position[position.length-1]-position[0];
        
  while(l < h) {
    var m = Math.floor(l + (h-l+1)/2);
    if (isOkay(position, m, b)) {
      l = m;
    } else {
      h = m-1;
    }
  }
  return l;
}
    
function isOkay(p, l, b) {
  if(--b == 0) return true;
  for(var i = 1, dist = 0; i < p.length; i++) {
    dist += p[i] - p[i-1];
    if(dist >= l) {
      if(--b == 0) return true;
      dist = 0;
    }
  }
  return false;
}

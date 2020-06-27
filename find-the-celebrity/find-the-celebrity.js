/**
 * Definition for knows()
 * 
 * @param {integer} person a
 * @param {integer} person b
 * @return {boolean} whether a knows b
 * knows = function(a, b) {
 *     ...
 * };
 */

/**
 * @param {function} knows()
 * @return {function}
 */
var solution = function(knows) {
    /**
     * @param {integer} n Total people
     * @return {integer} The celebrity
     */
  return function(n) {
    var candidates = new Map();
    for (var i=0;i<n;i++) {
      candidates.set(i, true);
    }
    for (var curr=0;curr<n;curr++) {
      if (!candidates.get(curr)) {
        continue;
      }
      var knowing = 0;
      for (var next=0;next<n;next++) {
        if (curr === next) {
          continue;
        }
        var k = knows(curr, next) ? 1 : 0;
        if (k === 1) {
          candidates.set(curr, false);
        } else {
          candidates.set(next, false);
        }
      }
    }
    console.log(candidates);
    var celebrity = [];
    for (const entry of candidates.entries()) {
      entry[1] && celebrity.push(entry[0]);
    }
    
    if (celebrity.length !== 1) {
      return -1;
    }
    
    var lastcheck = true;
    for (var i=0;i<n;i++) {
      if (i === celebrity[0]) {
        continue;
      }
      lastcheck = lastcheck && knows(i, celebrity[0]);
    }
    if (!lastcheck) {
      return -1;
    }
    return celebrity[0];
  };
};

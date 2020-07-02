/**
 * @param {number} n
 * @param {number[]} speed
 * @param {number[]} efficiency
 * @param {number} k
 * @return {number}
 */
var maxPerformance = function(n, speed, efficiency, k) {
  var maxperf = 0;
  var maxnum = n ** k;
  var cache = new Map();
  for (var i=0;i<maxnum;i++) {    
    var options = new Set();
    var d = i;
    while (d>0) {
      var r = d % n;
      options.add(r);
      d = Math.trunc(d / n);
    }
    if (options.size === 0) {
      options.add(0);
    }
    var keyarray = [...options];
    var sortedkey = keyarray.sort((a,b) => b-a);
    var key = sortedkey + "";
    if (cache.has(key)) {
      continue;
    }
    var k = sortedkey.slice(0, sortedkey.length-1) + "";
    var eff = Infinity;
    var spe = 0;
    let employee = sortedkey[sortedkey.length-1];
    if (cache.has(k)) {
      var entry = cache.get(k);
      eff = entry.eff;
      spe = entry.spe;
    }
    eff = Math.min(eff, efficiency[employee]);
    spe = spe + speed[employee];
    cache.set(key, {eff, spe});
    maxperf = Math.max(maxperf, eff*spe);
  }
  return maxperf;
};

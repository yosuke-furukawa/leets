/**
 * @param {number[]} quality
 * @param {number[]} wage
 * @param {number} K
 * @return {number}
 */
var mincostToHireWorkers = function(quality, wage, K) {
  var min = Infinity;
  var workers = quality.map((q, i) => ({quality:q, wage:wage[i]}));
  for (var i=0;i<quality.length;i++) {
    var baseq = quality[i];
    var basew = wage[i];
    var base = i;
    var k = K - 1;
    var cost = basew;
    var costPerQ = basew/baseq;
    
    var paygroups = workers.filter((worker, i) => {
      if (i === base) {
        return false;
      }
      var payment = worker.quality * costPerQ;
      worker.payment = payment;
      return worker.payment >= worker.wage;
    }).sort((a, b) => a.payment - b.payment);
    
    if (paygroups.length < k) {
      continue;
    }
    
    for (var p=0;p<k;p++) {
      cost += paygroups[p].payment;
    }
    
    min = Math.min(min, cost);
  }
  
  return min;
};

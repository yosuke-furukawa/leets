/**
 * @param {number[]} gas
 * @param {number[]} cost
 * @return {number}
 */
var canCompleteCircuit = function(gas, cost) {
  var length = gas.length;
  for (var i=0;i<length;i++) {
    var found = true;
    var tank = 0;
    var j = i;
    for (var count=0;count<length;count++) {
      var g = gas[j % length];
      var c = cost[j % length];
      tank += g - c;
      // console.log({tank, g, c, count, i, j})
      if (tank <= 0 && count < length-1) {
        found = false;
        break;
      } else if (count === length-1 && tank < 0) {
        found = false;
        break;
      }
      j = (j + 1) % length;
    }
    console.log(found);
    if (found) {
      return i;
    }
  }
  return -1;
};

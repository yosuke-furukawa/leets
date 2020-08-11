/**
 * @param {string} num
 * @return {boolean}
 */
const map = {
  "6": "9",
  "9": "6",
  "8": "8",
  "1": "1",
  "0": "0",
};
var isStrobogrammatic = function(num) {
  if (num.length % 2 === 1) {
    var half = (num.length/2)|0;
    if (num[half] !== "8" && num[half] !== "1" && num[half] !== "0") {
      return false;
    }
  }
  for (var i=0;i<num.length/2;i++) {
    var first = i;
    var last = num.length - i - 1;
    if (map[num[first]] !== num[last]) {
      return false;
    }
  }
  return true;
};

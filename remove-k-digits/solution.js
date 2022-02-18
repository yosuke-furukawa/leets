/**
 * @param {string} num
 * @param {number} k
 * @return {string}
 */
var removeKdigits = function(num, k) {
  var nums = [...num];
  var results = [];
  var len = nums.length - k;
  if (len === 0) {
    return "0";
  }
  for (var i=0;i<nums.length;i++) {
    var n1 = nums[i];
    var n2 = nums[i+1];
    if (n1 > n2) {
      nums.splice(i, 1);
      i = -1;
    }
    if (nums.length === len) {
      break;
    }
  }
  
  if (nums.length > len) {
    nums = nums.splice(0, len);
  }
  var y;
  for (var i=0;i<nums.length;i++) {
    if (y == null && nums[i] === '0') {
      continue;
    }
    y = i;
    break;
  }
  var str = nums.slice(i).join("");
  if (str.length === 0) {
    return "0";
  }
  return str;
};

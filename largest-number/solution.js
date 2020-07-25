/**
 * @param {number[]} nums
 * @return {string}
 */
var largestNumber = function(nums) {
  var comparator = (a, b, pre = "") => {
    if (a === "") {
      var barr = [...b];
      for (var ba of barr) {
        if (ba > pre) {
          return 1;
        } else if (ba < pre) {
          return -1;
        }
      }
      return 1;
    }
    if (b === "") {
      var aarr = [...a];
      for (var aa of aarr) {
        if (aa > pre) {
          return -1;
        } else if (aa < pre) {
          return 1;
        }
      }
      return -1;
    }
    a = String(a)
    b = String(b)
    if (b[0] < a[0]) {
      return -1
    } else if (b[0] > a[0]) {
      return 1
    } else {
      pre = pre > a[0] ? pre : a[0];
      return comparator(a.substring(1), b.substring(1), pre);
    }
  };
  console.log(nums.sort(comparator));
  var result = nums.sort(comparator).join("");
  if (result[0] === "0") {
    return "0";
  }
  return result;
};

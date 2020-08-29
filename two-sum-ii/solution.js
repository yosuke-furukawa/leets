/**
 * @param {number[]} numbers
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function(numbers, target) {
  for (var i=0;i<numbers.length-1;i++) {
    for (var j = numbers.length - 1;j>=i;j--) {
      var sum = numbers[i] + numbers[j];
      if (sum === target) {
        return [i+1, j+1];
      }
    }
  }
  return [];
};

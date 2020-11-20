/**
 * @param {number[]} nums
 */
var Solution = function(nums) {
  this.array = nums.map((n, i) => [n, i]).sort((a, b) => a[0]-b[0]);
};

/** 
 * @param {number} target
 * @return {number}
 */
Solution.prototype.pick = function(target) {
  var left = 0;
  var right = this.array.length - 1;
  var mid;
  while (left <= right) {
    mid = Math.floor((right + left) / 2);
    if (this.array[mid][0] < target) {
      left = mid + 1;
    } else if (this.array[mid][0] > target) {
      right = mid - 1;
    } else {
      break;
    }
  }
  console.log({left, right, mid});
  if (this.array[mid][0] !== target) {
    return -1;
  }
  var l = mid - 1;
  while (l >= 0) {
    if (this.array[l][0] < target) {
      l += 1;
      break;
    }
    l--;
  }
  
  var r = mid + 1;
  while (r <= this.array.length - 1) {
    if (this.array[r][0] > target) {
      r -= 1;
      break;
    }
    r++;
  }
  return this.array[Math.floor(Math.random() * (r-l) + l)][1];
};

/** 
 * Your Solution object will be instantiated and called as such:
 * var obj = new Solution(nums)
 * var param_1 = obj.pick(target)
 */

/**
 * @param {string} time
 * @return {string}
 */
var nextClosestTime = function(time) {
  var result = [...time];
  var nums = [...time];
  delete nums[2];
  nums = nums.sort((a, b) => (+a) - (+b));
  
  var lastMin = +time[time.length-1];
  var nextLastMin = nums.find((n) => +n > lastMin);
  if (nextLastMin != null) {
    result[result.length-1] = nextLastMin;
    return result.join("");
  }
  
  var tenMin = +time[time.length-2];
  if (tenMin !== 5) {
    var nextTenMin = nums.find((n) => +n > tenMin && +n < 6);
    if (nextTenMin != null) {
      result[result.length-2] = nextTenMin;
      result[result.length-1] = nums[0];
      return result.join("");  
    }
  }
  var tenHour = +time[0];
  var oneHour = +time[1];

  if (tenHour < 2) {
    if (oneHour < 9) {
      var nextOneHour = nums.find((n) => +n > oneHour);
      if (nextOneHour != null) {
        result[1] = nextOneHour;
        result[result.length-2] = nums[0];
        result[result.length-1] = nums[0];
        return result.join("");
      }
    }
    var nextTenHour = nums.find((n) => +n > tenHour && +n < 3);
    if (nextTenHour != null) {
      result[0] = nextTenHour;
      result[1] = nums[0];
      result[result.length-2] = nums[0];
      result[result.length-1] = nums[0];
      return result.join("");
    }
  }
  
  if (tenHour === 2) {
    if (oneHour < 4) {
      var nextOneHour = nums.find((n) => +n > oneHour && +n < 4);
      if (nextOneHour != null) {
        result[1] = nextOneHour;
        result[result.length-2] = nums[0];
        result[result.length-1] = nums[0];
        return result.join("");
      }
    }
  }
  
  result[0] = nums[0];
  result[1] = nums[0];
  result[result.length-2] = nums[0];
  result[result.length-1] = nums[0];
  return result.join("");
};

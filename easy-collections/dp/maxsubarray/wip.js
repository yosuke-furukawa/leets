var midSum = function(nums) {
    var low = 0;
    var high = nums.length;
    var mid = Math.ceil((high - low) /2);
    var sum = 0;
    var lsum = -Infinity;
    var rsum = -Infinity;
    
    for (var i = mid-1; i>=0; i--) {
        sum += nums[i];
        if (sum > lsum) {
            lsum = sum;
        }
    }
    sum = 0;
    for (var j = mid; j<high; j++) {
        sum += nums[j];
        if (sum > rsum) {
            rsum = sum;
        }
    }
    var result =0;
     if (lsum < 0 && rsum < 0) {
        if (lsum < rsum) {
            result = rsum
        } else {
            result = lsum
        }
         return result
     }
    if (lsum < 0) {
        result = rsum;
    } else if (rsum < 0) {
        result = lsum;
    } else {
        result = lsum + rsum;
    }
    return result;
}

var leftSum = function(nums) {
    var low = 0;
    var high = nums.length;
    var mid = Math.ceil((high - low) /2);
    var sum = 0;
    var lsum = -Infinity
    
    for (var i = 0; i<mid; i++) {
        
        sum += nums[i];
        if (sum > lsum) {
            lsum = sum;
        }
    }
    return lsum;
}

var rightSum = function(nums) {
    var low = 0;
    var high = nums.length;
    var mid = Math.ceil((high - low) /2);
    var sum = 0;
    var rsum = -Infinity
    
    for (var i = mid; i<high; i++) {
        
        sum += nums[i];
        if (sum > rsum) {
            rsum = sum;
        }
        console.log({rsum, num:nums[i]})
    }
    return rsum;
}

/**
 * @param {number[]} nums
 * @return {number}
 */
var maxSubArray = function(nums) {
    var lsum = leftSum(nums);
    var rsum = rightSum(nums);
    var msum = midSum(nums);
    return Math.max(lsum, rsum, msum);
};

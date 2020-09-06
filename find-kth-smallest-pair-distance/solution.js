
/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var smallestDistancePair = function(nums, k) {
  nums.sort((a,b)=>a-b);
		
  let min=Infinity, max=nums[nums.length-1]-nums[0];
  for (var i = 0; i < nums.length-1; i++) {
    min = Math.min(min, nums[i+1]-nums[i]);
  }
		
  function countLessThan(nums, target){
    let right = 0, count=0;
    for (var i = 0; i < nums.length; i++) {
    	while(nums[i]-nums[right]>target && right<=i) right++;
    	count+=(i-right);
    }
    return count;
  }
		
  while(min+1<max){
    let mid = Math.floor((min+max)/2);
    let count = countLessThan(nums, mid);
    if(count>countLessThan(nums,mid-1)&&count===k) return mid;
    if(count<k) min=mid;
    else max=mid;
  }
  if(countLessThan(nums,min)>=k) return min;
  return max;
};

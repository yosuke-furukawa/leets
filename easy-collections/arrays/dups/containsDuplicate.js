var containsDuplicate = function(nums) {
  var a = nums.sort();
  for (var i=0; i<a.length; i++) {
    if (a[i] === a[i+1]) {
      return true;
    }
  }
  return false;
};

console.log(containsDuplicate([1,2,3,1]));
console.log(containsDuplicate([1,2,3,4]));
console.log(containsDuplicate([1,1,1,3,3,4,3,2,4,2]));

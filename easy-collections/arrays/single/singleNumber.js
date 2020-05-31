var singleNumber = function(nums) {
  var a = nums.sort();
  var c = 0;
  for(var i=0; i<a.length; i++) {
    if (a[i] !== a[i+1] && c === 0) {
      return a[i];
    }
    if (a[i] === a[i+1]) {
      c++;
    }
    if (a[i] !== a[i+1]) {
      c=0;
    }
  }
}

console.log(singleNumber([2,2,1]));
console.log(singleNumber([4,1,2,1,2]));

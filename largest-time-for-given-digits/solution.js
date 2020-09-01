/**
 * @param {number[]} A
 * @return {string}
 */
var isValid = (time, a) => {
  var newTime = [...time];
  newTime.push(a);
  var [h1, h2, m1, m2] = newTime;
  var hourValid = false;
  if (h1 > 3) {
    return false;
  }
  if (h1 === 2) {
    if (h2 >= 4) {
      return false;
    }
  }
  
  if (m1 && m1 > 5) {
    return false;
  }
  return true;
}


var backtrack = (A, N, time) => {
  var B = A.filter((a) => a <= N[time.length]);
  for (var i=0;i<B.length;i++) {
    var a = B[i];
    if (isValid(time, a)) {
      time.push(a);
      if (time.length === 4) {
        return time;
      } else {
        var newA = [...A];
        newA.splice(A.indexOf(a), 1);
        var r = backtrack(newA, N, time);
        if (r) {
          return r;
        }
      }
      time.pop();
    }
  }
}

var largestTimeFromDigits = function(A) {
  var time = [];
  var result = backtrack(A.sort((a, b) => b-a), [2,9,5,9], time);
  if (result) {
    result.splice(2, 0, ":");
    return result.join("");
  }
  return "";
};

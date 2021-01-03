/**
 * @param {number} n
 * @return {number}
 */
var countArrangement = function(n) {
  var list = Array(n).fill(0).map((_, i) => i+1);
  
  var result = new Set();
  var backtrack = (temps, list) => {
    if (list.length === 0) {
      result.add(temps.join(","));
      return;
    }
    
    var index = temps.length + 1;
    for (var i=0;i<list.length;i++) {
      var num = list[i];
      if (index % num === 0 || num % index === 0) {
        var array = [...temps];
        array.push(num);
        list.splice(i, 1);
        backtrack(array, list);
        list.splice(i, 0, num);
      }
    }
  }
  
  backtrack([], list);
  return result.size;
};

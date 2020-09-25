/**
 * @param {string[]} list1
 * @param {string[]} list2
 * @return {string[]}
 */
var findRestaurant = function(list1, list2) {
  var map1 = new Map();
  
  for (var i1=0;i1<list1.length;i1++) {
    map1.set(list1[i1], i1);
  }
  
  var minsum = Infinity;
  var map2 = new Map();
  for (var i2=0;i2<list2.length;i2++) {
    var l2 = list2[i2];
    if (map1.has(l2)) {
      var temp = map1.get(l2) + i2;
      if (map2.has(temp)) {
        map2.get(temp).push(l2);      
      } else {
        map2.set(temp, [l2]);        
      }
      if (minsum > temp) {
        minsum = temp;
      }
    } 
  }
  
  return map2.get(minsum);
};

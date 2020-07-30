/**
 * @param {number[]} tree
 * @return {number}
 */
var totalFruit = function(tree) {
  var count = 0;
  for (var i=0;i<tree.length;i++) {
    if (count > tree.length/2) {
      break;
    }
    var set = new Set();
    var first = tree[i];
    set.add(first);
    var temp = 1;
    for (var j=i+1;j<tree.length;j++) {
      var second = tree[j];
      if (set.has(second)) {
        temp++;
      } else if (set.size < 2) {
        set.add(second);
        temp++;
      } else {
        break;
      }
    }
    count = Math.max(count, temp);
  }
  return count;
};

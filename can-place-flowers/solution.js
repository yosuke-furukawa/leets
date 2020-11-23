/**
 * @param {number[]} flowerbed
 * @param {number} n
 * @return {boolean}
 */
var canPlaceFlowers = function(flowerbed, n) {
  var count = 0;
  for (var i=0;i<flowerbed.length;i++) {
    var f1 = flowerbed[i-1] ?? 0;
    var f2 = flowerbed[i];
    var f3 = flowerbed[i+1] ?? 0;
    if (f1 === 0 && f1 === f2 && f2 === f3) {
      count++;
      i++;
    }
  }
  return count >= n;
};

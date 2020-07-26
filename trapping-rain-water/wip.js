/**
 * @param {number[]} height
 * @return {number}
 */
var trap = function(height) {
  var maxheight = Math.max(...height);
  var watermark = 1;
  var rain = 0;
  while(watermark <= maxheight) {
    var left = height.findIndex((h) => h >= watermark);
    var raincandidate = 0;
    for (var i=left; i<=height.length;i++) {
      var h = height[i];
      if (watermark > h) {
        raincandidate++;
      }
      if (raincandidate > 0 && h >= watermark) {
        rain += raincandidate;
        raincandidate=0;
      }
    }
    watermark++;
  }
  return rain;
};

/**
 * @param {number[][]} image
 * @param {number} sr
 * @param {number} sc
 * @param {number} newColor
 * @return {number[][]}
 */
var floodFill = function(image, sr, sc, newColor) {
  var currentColor = image[sr][sc];
  var helper = (nr, nc) => {
    if (image[nr]?.[nc] == null) {
      return;
    }
    if (image[nr][nc] !== currentColor) {
      return;
    }
    if (image[nr][nc] === newColor) {
      return;
    }
    
    image[nr][nc] = newColor;
    helper(nr-1, nc);
    helper(nr, nc+1);
    helper(nr, nc-1);
    helper(nr+1, nc);
    
  };
  helper(sr, sc);
  return image;
};

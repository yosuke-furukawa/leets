/**
 * @param {number[]} arr
 * @param {number[][]} pieces
 * @return {boolean}
 */
var canFormArray = function(arr, pieces) {
  for (var piece of pieces) {
    var deleted = arr.splice(arr.indexOf(piece[0]), piece.length, 0);
    for (var i=0;i<piece.length;i++) {
      if (piece[i] !== deleted[i]) {
        return false;
      }
    }
  }
  return arr.filter((n) => n>0).length === 0
};

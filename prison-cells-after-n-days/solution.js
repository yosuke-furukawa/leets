/**
 * @param {number[]} cells
 * @param {number} N
 * @return {number[]}
 */
var prisonAfterNDays = function(cells, N) {
  var map = new Map();
  var cellkey = cells.join("");
  var array = [];
  for (var i=0;i<N;i++) {
    var nextcells = new Array(cells.length);
    if (map.has(cellkey)) {
      var cycle = i - map.get(cellkey);
      var rest = (N - i) % cycle;
      return array[map.get(cellkey)+rest];
    }
    
    for (var c=0;c<cells.length;c++) {
      var prev = cells[c-1];
      var cell = cells[c];
      var next = cells[c+1];
      if (prev === next) {
        nextcells[c] = 1;
      } else {
        nextcells[c] = 0;
      }
    }

    var nextkey = nextcells.join("");
    array.push(cells);
    map.set(cellkey, array.length-1);
    cells = nextcells;
    cellkey = nextkey;
  }
  return cells;
};

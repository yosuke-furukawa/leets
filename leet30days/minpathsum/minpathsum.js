/**
 * @param {number[][]} grid
 * @return {number}
 */
var minPathSum = function(grid) {
    for (var y=0;y<grid.length;y++) {
        for (var x=0;x<grid[y].length;x++) {
            if (x===0 && y===0) continue;
            var a = grid[y][x];
            var y1 = 0 <= y-1 ? grid[y-1][x] : Infinity;
            var x1 = 0 <= x-1 ? grid[y][x-1] : Infinity;
            var sum = a + Math.min(y1, x1);
            grid[y][x] = sum;
        }
    }
    var xlen = grid[grid.length-1].length-1;
    return grid[grid.length-1][xlen];
};

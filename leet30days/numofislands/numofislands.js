var walk = function(x, y, grid, visited, islands) {
    if (0 <= y+1 && y+1 < grid.length && grid[y+1][x] === "1" && !visited[`${x},${y+1}`]) {
        visited[`${x},${y+1}`] = islands;
        walk(x, y+1, grid, visited, islands);
    }
    if (0 <= x+1 &&x+1 < grid[y].length && grid[y][x+1] === "1" && !visited[`${x+1},${y}`]) {
        visited[`${x+1},${y}`] = islands;
        walk(x+1, y, grid, visited, islands);
    }
    if (0 <= y-1 && y-1 < grid.length && grid[y-1][x] === "1" && !visited[`${x},${y-1}`]) {
        visited[`${x},${y-1}`] = islands;
        walk(x, y-1, grid, visited, islands);
    }
    if (0 <= x-1 && x-1 < grid[y].length && grid[y][x-1] === "1" && !visited[`${x-1},${y}`]) {
        visited[`${x-1},${y}`] = islands;
        walk(x-1, y, grid, visited, islands);
    }
}

/**
 * @param {character[][]} grid
 * @return {number}
 */
var numIslands = function(grid) {
    var islands = 0;
    var visited = {};
    var x = 0;
    var y = 0;
    while(y<grid.length) {
        if (grid[y][x] === "1") {
            if (!visited[`${x},${y}`]) {
                islands++;
                visited[`${x},${y}`] = islands;
            }
            walk(x,y,grid,visited,islands);
        }
        if (x< grid[y].length) {
           x++;
        } else {
           x=0;
           y++;
        }
    }
    return islands;
};

/**
 * Initialize your data structure here.
        @param width - screen width
        @param height - screen height 
        @param food - A list of food positions
        E.g food = [[1,1], [1,0]] means the first food is positioned at [1,1], the second is at [1,0].
 * @param {number} width
 * @param {number} height
 * @param {number[][]} food
 */
var SnakeGame = function(width, height, food) {
  this.height = height;
  this.width = width;
  this.score = 0;
  this.food = food;
  this.head = [0, 0];
  this.body = [];
};

/**
 * Moves the snake.
        @param direction - 'U' = Up, 'L' = Left, 'R' = Right, 'D' = Down 
        @return The game's score after the move. Return -1 if game over. 
        Game over when snake crosses the screen boundary or bites its body. 
 * @param {string} direction
 * @return {number}
 */
SnakeGame.prototype.move = function(direction) {
  // console.log(this.map);
  var prevPos = [...this.head];
  if (direction === "U") {
    this.head[0] -= 1; 
  } else if (direction === "L") {
    this.head[1] -= 1;
  } else if (direction === "R") {
    this.head[1] += 1;
  } else if (direction === "D") {
    this.head[0] += 1;
  }
  
  // console.log(this.head, this.body);
  if (this.head[0] < 0 || this.head[0] >= this.height) {
    // console.log("here?")
    return -1;
  }
  
  if (this.head[1] < 0 || this.head[1] >= this.width) {
    // console.log("here?")
    return -1;
  }
  
  if (this.food[0]?.[0] === this.head[0] && this.food[0]?.[1] === this.head[1]) {
    this.score++;
    var prev = prevPos;
    if (this.body.length > 0) {
      prev = this.body.pop();
      this.body.unshift(prevPos);
    }
    this.body.push(prev);
    if (this.food.length > 0) {
      this.food.shift();
    }
  } else {
    if (this.body.length > 0) {
      this.body.pop();
      this.body.unshift(prevPos);
    }
  }
  for (var p of this.body) {
    if (p[0] === this.head[0] && p[1] === this.head[1]) {
      return -1;
    }
  }
  return this.score;
};

/** 
 * Your SnakeGame object will be instantiated and called as such:
 * var obj = new SnakeGame(width, height, food)
 * var param_1 = obj.move(direction)
 */

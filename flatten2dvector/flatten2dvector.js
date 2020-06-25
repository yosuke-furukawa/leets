/**
 * @param {number[][]} v
 */
var Vector2D = function(v) {
  this.vector = v.filter((arr) => arr.length > 0);
  this.inner = 0;
  this.outer = 0;
};

Vector2D.prototype.incr = function() {
  if (this.vector[this.outer] instanceof Array) {
    if (this.inner >= this.vector[this.outer].length-1) {
      this.inner = 0;
      this.outer++;
    } else {
      this.inner++;
    }
  } else {
    this.outer++;
  }
}

/**
 * @return {number}
 */
Vector2D.prototype.next = function() {
  var result;
  while (result == null) {
    var outer = this.vector[this.outer];
    if (outer instanceof Array) {
      result = outer[this.inner];
    } else {
      result = outer;
    }
    this.incr();
  }
  return result;
};

/**
 * @return {boolean}
 */
Vector2D.prototype.hasNext = function() {
  return this.outer < this.vector.length || (this.inner < this.vector[this.outer]?.length);
};

/** 
 * Your Vector2D object will be instantiated and called as such:
 * var obj = new Vector2D(v)
 * var param_1 = obj.next()
 * var param_2 = obj.hasNext()
 */

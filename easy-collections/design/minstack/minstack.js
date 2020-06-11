/**
 * initialize your data structure here.
 */
var MinStack = function() {
    this.data = [];
    this.minstacks = [];
};

/** 
 * @param {number} x
 * @return {void}
 */
MinStack.prototype.push = function(x) {
    this.data.unshift(x);
    if (this.minstacks.length > 0) {
        var min = Math.min(x, this.minstacks[0]);
        this.minstacks.unshift(min);        
    } else {
        this.minstacks.unshift(x);
    }
};

/**
 * @return {void}
 */
MinStack.prototype.pop = function() {
    this.minstacks.shift();
    return this.data.shift();
};

/**
 * @return {number}
 */
MinStack.prototype.top = function() {
    return this.data[0];
};

/**
 * @return {number}
 */
MinStack.prototype.getMin = function() {
    return this.minstacks[0];
};

/** 
 * Your MinStack object will be instantiated and called as such:
 * var obj = new MinStack()
 * obj.push(x)
 * obj.pop()
 * var param_3 = obj.top()
 * var param_4 = obj.getMin()
 */

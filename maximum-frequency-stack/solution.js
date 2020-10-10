var FreqStack = function() {
  this.freqMap = new Map();
  this.setMap = new Map();
  this.maxfreq = 0;
};

/** 
 * @param {number} x
 * @return {void}
 */
FreqStack.prototype.push = function(x) {
  let freqs = this.freqMap.get(x) ?? 0;
  freqs++;
  this.freqMap.set(x, freqs);
  
  if (freqs > this.maxfreq) {
    this.maxfreq = freqs;
  }
  
  
  let sets = this.setMap.get(freqs);
  
  if (sets != null) {
    sets.push(x);
  } else {
    this.setMap.set(freqs, [x]);
  }
};

/**
 * @return {number}
 */
FreqStack.prototype.pop = function() {
  const value = this.setMap.get(this.maxfreq);
  const top = value.pop();
  
  this.freqMap.set(top, this.freqMap.get(top)-1);
  if (this.setMap.get(this.maxfreq).length === 0) {
    this.maxfreq--;  
  } 
  return top; 
};

/** 
 * Your FreqStack object will be instantiated and called as such:
 * var obj = new FreqStack()
 * obj.push(x)
 * var param_2 = obj.pop()
 */

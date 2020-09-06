/**
 * @param {character[]} letters
 * @param {character} target
 * @return {character}
 */

var nextGreatestLetter = function(letters, target) {
  var chars = new Map();
  var firstletter = letters[0];
  var lastletter = letters[letters.length-1];
  for (var i=0; i<letters.length-1;i++) {
    var letter1 = letters[i];
    var letter2 = letters[i+1];
    chars.set(letter1, letter2);
  }
  chars.set(lastletter, firstletter);
  // console.log(chars);
  
  var len = letters.length;
  var left = 0;
  var right = letters.length-1;
  var mid = left + Math.floor((right - left)/2);
  while (left + 1 < right) {
    if (letters[mid] === target) {
      return chars.get(target);
    }
    
    if (letters[mid].charCodeAt() < target.charCodeAt()) {
      left = mid;
    } else {
      right = mid;
    }
     mid = left + Math.floor((right - left)/2);
  }
  
  if (letters[left] === target) {
    mid = left;
    return chars.get(target);
  }
  
  if (letters[right] === target) {
    mid = right;
    return chars.get(target);
  }
  
  if (Math.abs(letters[left].charCodeAt() - target.charCodeAt()) < Math.abs(letters[mid].charCodeAt() - target.charCodeAt())) {
    mid = left;
  }
  
  if (Math.abs(letters[right].charCodeAt() - target.charCodeAt()) < Math.abs(letters[mid].charCodeAt() - target.charCodeAt())) {
    mid = right;
  }
  
  // console.log(letters[mid], target, (mid+1)%len);
  if (letters[mid].charCodeAt() > target.charCodeAt()) {
    return letters[mid];
  } else {
    return chars.get(letters[mid]);
  }  
};

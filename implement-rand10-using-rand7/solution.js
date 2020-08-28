/**
 * The rand7() API is already defined for you.
 * var rand7 = function() {}
 * @return {number} a random integer in the range 1 to 7
 */
var rand10 = function() {
  var rand40 = 40;
  while(rand40 >= 40){
    rand40 = (rand7() - 1) * 7 + (rand7() - 1);
  }
  return rand40 % 10 + 1;
};

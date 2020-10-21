/**
 * @param {number[]} asteroids
 * @return {number[]}
 */
var asteroidCollision = function(asteroids) {
  if (asteroids.length <= 1) {
    return asteroids;
  }
  var results = [];  
  while (asteroids.length > 0) {
    // console.log({asteroids, results});
    var asteroid = asteroids.shift();
    var peek = results[results.length - 1];
    if (peek == null) {
      results.push(asteroid);
      continue;
    }
    if (asteroid > 0) {
      results.push(asteroid);
    } else {
      // console.log(results, asteroid);
      var append = false;
      for (var r=results.length-1;r>=0;r--) {
        if (results[r] > 0) {
          if (-results[r] > asteroid) {
            results.pop();
            append = true;
            r++;
          } else if (-results[r] === asteroid) {
            // console.log("pop?");
            append = false;
            results.pop();
            break;
          } else if (-results[r] < asteroid) {
            append = false;
            break;
          }
        } else {
          append = true;
        }
      }
      
      if (append) {
        results.push(asteroid);
      }
    }
  }
  return results;
};

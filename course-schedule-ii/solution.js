/**
 * @param {number} numCourses
 * @param {number[][]} prerequisites
 * @return {number[]}
 */
var findOrder = function(numCourses, prerequisites) {
  //console.log(prerequisites);
  var map = new Map();
  var set = new Set();
  var result = [];
  for (var requires of prerequisites) {
    var course = requires[0];
    var require = requires[1];
    var courses = map.get(course);
    if (courses) {
      if (!courses.includes(require)) {
        courses.push(require);
        map.set(course, courses);
      }
    } else {
      map.set(course, [require]);
    }
  }
  var visited = new Set();
  var direct = true;
  var dfs = (courses, map) => {
    if (courses == null) {
      return;
    }
    
    for (var course of courses) {
      //console.log({set, course, visited});
      if (set.has(course)) {
        continue;
      }
      
      if (visited.has(course)) {
        direct = false;
        return;
      }
      visited.add(course);
      dfs(map.get(course), map);
      if (direct) {
        if (!set.has(course)) {
          result.push(course);
          set.add(course);
        }
        
      }
    }
    //console.log({t:1, direct})
    return;
  }

  for (var key of map.keys()) {
    dfs(map.get(key), map);
    if (!direct) {
      return [];
    }
    //console.log({t:2, direct})
    if (!set.has(key)) {
      result.push(key);
      set.add(key);
    }
  }
  

  var sort = [...result].sort((a, b) => a-b);
  for (var i=0;i<numCourses;i++) {
    if (i != sort[i]) {
      result.push(i);
      sort.splice(i, 0, i);
    }
  }

  return result;
};

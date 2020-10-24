/**
 * @param {string} s
 * @param {string[]} words
 * @return {number[]}
 */
var findSubstring = function(s, words) {
  const ac = new AhoCorasick(words);
  var wordsLen = words.map((w) => w.length).reduce((acc, curr) => acc + curr);
  
  const matches = ac.match(s);
  // console.log(matches);
  var results = [];
  for (var i=0;i<matches.length;i++) {
    var ws = new Map();
    for (const w of words) {
      if (ws.has(w)) {
        ws.set(w, ws.get(w) + 1);
        continue;
      }
      ws.set(w, 1);
    }
    let v = ws.get(matches[i][0]);
    if (v === 1) {
      ws.delete(matches[i][0]);
    } else {
      ws.set(matches[i][0], v-1);
    }
    if (ws.size === 0) {
      results.push(matches[i][1]);
      continue;
    }
    var ii = i;
    for (var j=i+1;j<matches.length;j++) {
      // console.log(ws);
      if (matches[ii][2]+1 === matches[j][1]) {  
        if (!ws.has(matches[j][0])) {
          break;
        }

        let v = ws.get(matches[j][0]);
        if (v === 1) {
          ii = j;
          ws.delete(matches[j][0]);
        } else {
          ii = j;
          ws.set(matches[j][0], v-1);
        }
        
        if (ws.size === 0) {
          results.push(matches[i][1]);
          break;
        }
      }
    }
  }
  return results;
};

class State {
  constructor(id) {
    this.id = id;
    this.next = {};
  }

  key(x) {
    return this.next[x];
  }
}
class AhoCorasick {
  constructor(words) {
    this.state = [ new State(0) ];
    this.output = [[]];
    this.makeGoto(words);
    this.makeFailure();
  }

  makeGoto(words) {
    for (const word of words) {
      let curr = this.state[0];
      for (const c of [...word]) {
        if (curr.key(c) == null) {
          const next = new State(this.state.length);
          curr.next[c] = next;

          this.state.push(next);
          this.output.push([]);
        }
        curr = curr.next[c];
      }
      const id = curr.id;
      this.output[id].push(word);
    }
  }

  makeFailure() {
    const failure = new Array(this.state.length).fill(0);
    const queue = [ 0 ];
    while (queue.length > 0) {
      const s = queue.shift();
      for (const c of Object.keys(this.state[s].next)) {
        const next = this.g(s, c);

        if (next != null) {
          queue.push(next);
        }

        if (s != 0) {
          let f = failure[s];
          while (this.g(f, c) == null) {
            f = failure[f];
          }
          failure[next] = this.g(f, c);

          this.output[next].concat(this.output[failure[next]]);
        }
      }
    }
    this.failure = failure;
  }

  g(s, c) {
    if (c in this.state[s].next) {
      return this.state[s].next[c].id;
    } else {
      if (s === 0) {
        return 0;
      } else {
        return;
      }
    }
  }

  match(query) {
    let s = 0;
    const results = [];
    for (var i=0;i<query.length;i++) {
      while (this.g(s, query[i]) == null) {
        s = this.failure[s];
      } 

      s = this.g(s, query[i]);
      
      for (const c of this.output[s]) {
        if (results[results.length - 1]?.[1] !== i-c.length+1) {
          results.push([c, i-c.length+1, i]);
        }
      }
    }
    return results;
  }
}

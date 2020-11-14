let minimumJumps = (A, R, L, T, seen = new Set(), best = Infinity) => {
    let forbidden = new Set(A);
    let go = (i = 0, steps = 0, backwards = false) => {
        if (forbidden.has(i))              // 🚫 forbidden
            return;
        let key = `${i},${backwards}`;
        if (seen.has(key))                 // 👀 seen
            return;
        seen.add(key);
        if (i == T) {
            best = Math.min(best, steps);  // 🎯 minimum steps to reach target
            return;
        }
        if (i < 0 || 4000 < i)             // 🛑 out of bounds
            return;
        go(i + R, steps + 1, false);       // 🚀 DFS explore 👉 to-the-right
        if (!backwards)
            go(i - L, steps + 1, true);    // 🚀 DFS explore 👈 to-the-left
    };
    go();
    return best < Infinity ? best : -1;
};

function findMissingAndRepeatedValues(grid: number[][]): number[] {
    const range = [1, Math.pow(grid.length, 2)];

    let [missing, dupe] = [null, null];

    let set = new Set();

    for (let i = 0; i < grid[0].length; i++) {
        for (let j = 0; j < grid[0].length; j++) {
            let val = grid[i][j];
            if (set.has(val)) {
                dupe = val;
            }

            set.add(val)
        }
    }

    for (let i = 1; i <= range[1]; i ++) {
        if(!set.has(i)) {
            missing = i;
            break
        }
    }

    return [dupe, missing]
};
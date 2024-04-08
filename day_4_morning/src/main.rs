struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

#[derive(Clone)]
struct Grid {
    x_coords: Vec<u32>,
    y_coords: Vec<u32>,
}

impl <'a> IntoIterator for &'a Grid {
    type Item = (u32, u32);
    type IntoIter = GridIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        GridIter { grid: self, i: 0, j: 0 }
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    i: usize,
    j: usize,
}

impl <'a> Iterator for GridIter<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }
        let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
        self.i += 1;
        res
    }
}


fn main() {
    let fib = Fibonacci { curr: 0, next: 1 };
    for (i, n) in fib.enumerate().take(10) {
        println!("fib({i}): {n}");
    }

    // The example iterates over all combinations of x and y coordinates.
    let grid = Grid { x_coords: vec![3, 5, 7, 9], y_coords: vec![10, 20, 30, 40] };
    for (x, y) in &grid {
        println!("point = {x}, {y}");
    }

    for (x, y) in &grid {
        println!("point = {x}, {y}");
    }

    let primes = vec![2, 3, 5, 7];
    let prime_squares = primes.into_iter().map(|p| p * p).collect::<Vec<_>>();
    println!("prime_squares: {prime_squares:?}");
}
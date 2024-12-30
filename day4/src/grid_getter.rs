pub struct GridGetter<'a> {
    row: u32,
    col: u32,
    grid: &'a Vec<Vec<char>>,
}

impl<'a> GridGetter<'a> {
    pub fn new(grid: &'a Vec<Vec<char>>, row: u32, col: u32) -> GridGetter<'a> {
        GridGetter { row, col, grid }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&char> {
        self.grid
            .get((self.row as i32 - y) as usize)
            .and_then(|row| row.get((self.col as i32 + x) as usize))
    }

    pub fn seq_matches(&self, seq: &[(i32, i32)], expected: &str) -> bool {
        let expected_chars = expected.chars().collect::<Vec<char>>();

        for i_xy in seq.iter().enumerate() {
            let i = i_xy.0;
            let xy = i_xy.1;

            match self.get(xy.0, xy.1) {
                None => return false,
                Some(ch) => {
                    if *ch != expected_chars[i] {
                        return false;
                    }
                }
            }
        }

        true
    }
}

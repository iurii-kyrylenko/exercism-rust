pub struct PascalsTriangle {
    count: usize,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            count: row_count as usize,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut data = Vec::with_capacity(self.count);

        for n in 0..self.count {
            PascalsTriangle::add_row(&mut data, n)
        }

        data
    }

    fn add_row(data: &mut Vec<Vec<u32>>, n: usize) {
        if n < 2 {
            data.push(vec![1; n + 1]);
            return;
        }

        let prev_row = &data[n - 1];
        let mut new_row = vec![1; n + 1];

        for i in 1..n {
            new_row[i] = prev_row[i - 1] + prev_row[i];
        }

        data.push(new_row);
    }
}

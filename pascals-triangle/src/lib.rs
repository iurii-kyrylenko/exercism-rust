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
        for n in 1..self.count + 1 {
            PascalsTriangle::add_row(&mut data, n)
        }

        data
    }

    fn add_row(data: &mut Vec<Vec<u32>>, n: usize) {
        if n < 3 {
            data.push(vec![1; n]);
            return;
        }

        let prev_row = &data[n - 2];
        let mut new_row = vec![1; n];

        for i in 1..n - 1 {
            new_row[i] = prev_row[i - 1] + prev_row[i];
        }

        data.push(new_row);
    }
}

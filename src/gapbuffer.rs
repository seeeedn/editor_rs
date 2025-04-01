use std::usize;

const INIT_SIZE: usize = 1024;

pub struct GapBuffer {
    data: Vec<char>,
    total: usize,
    gap_start: usize,
    gap_end: usize
}

impl GapBuffer {
    fn init_buffer(size: usize) -> Self {
        let mut buffer: Vec<char> = vec![' '; size];
        Self {
            data: buffer,
            total: INIT_SIZE,
            gap_start: 0,
            gap_end: INIT_SIZE - 1 
        }
    }

    fn expand_buffer(&mut self) {
        let new_total = self.total * 2;
        let after_buffer = self.data.len() - self.gap_end;

        self.data.resize(new_total, ' ');

        let new_gap_end = new_total - after_buffer;

        self.data.copy_within(self.gap_end.., new_gap_end);

        self.gap_end = new_gap_end;
        self.total = new_total;
    }

    fn insert_char(&mut self, ch: char) {
        if self.gap_start == self.gap_end - 1 {
            self.expand_buffer();
        }

        if self.gap_start < self.total {
            self.data[self.gap_start] = ch;
            self.gap_start += 1;
        }
    }

    fn delete_char(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
        }
    }

    fn delete_char_forward(&mut self) {
        if self.gap_end < self.total - 1 {
            self.gap_end += 1;
        }
    }

    fn move_right(&mut self, offset: usize) {
        if offset <= self.gap_start || self.gap_end + offset >= self.total {
            return;
        }

        self.data.copy_within(self.gap_end + 1..self.gap_end + offset, self.gap_start);

        self.gap_start += offset;
        self.gap_end += offset;
    }

    fn move_left(&mut self, offset: usize) {
        if self.gap_start < offset {
            return;
        }

        self.data.copy_within(self.gap_start - offset..self.gap_start - 1, self.gap_end - offset);

        self.gap_start -= offset;
        self.gap_end -= offset;
    }

    fn move_gap(&mut self, offset: usize) {
        
    }
}
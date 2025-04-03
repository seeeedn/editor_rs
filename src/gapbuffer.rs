use std::io;
use std::{fs, usize, vec};
use std::fs::File;

const INIT_SIZE: usize = 1024;

pub struct GapBuffer {
    data: Vec<char>,
    total: usize,
    gap_start: usize,
    gap_end: usize
}

impl GapBuffer {
    pub fn init_buffer() -> Self {
        let mut buffer = vec![' '; INIT_SIZE];
        Self {
            data: buffer,
            total: INIT_SIZE,
            gap_start: 0,
            gap_end: INIT_SIZE - 1 
        }
    }

    pub fn expand_buffer(&mut self) {
        let new_total = self.total * 2;
        let after_buffer = self.data.len() - self.gap_end;
        
        self.data.resize(new_total, ' ');

        let new_gap_end = new_total - after_buffer;

        self.data.copy_within(self.gap_end..self.total, new_gap_end);

        self.gap_end = new_gap_end;
        self.total = new_total;
    }

    pub fn insert_char(&mut self, ch: char) {
        if self.gap_start == self.gap_end - 1 {
            self.expand_buffer();
        }

        if self.gap_start < self.total {
            self.data[self.gap_start] = ch;
            self.gap_start += 1;
        }
    }

    pub fn delete_char(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
        }
    }

    pub fn delete_char_forward(&mut self) {
        if self.gap_end < self.total - 1 {
            self.gap_end += 1;
        }
    }

    pub fn move_right(&mut self, offset: usize) {
        if offset <= self.gap_start || self.gap_end + offset >= self.total {
            return;
        }

        self.data.copy_within(self.gap_end + 1..self.gap_end + offset, self.gap_start);

        self.gap_start += offset;
        self.gap_end += offset;
    }

    pub fn move_left(&mut self, offset: usize) {
        if self.gap_start < offset {
            return;
        }

        self.data.copy_within(self.gap_start - offset..self.gap_start - offset + 1, self.gap_end - offset);

        self.gap_start -= offset;
        self.gap_end -= offset;
    }

    pub fn move_gap(&mut self, offset: usize) {
        if offset > self.gap_start {
            self.move_right(offset);
        }
        else if offset < self.gap_start {
            self.move_left(offset);
        }
    }

    pub fn data_to_string(&self) -> String {
        let mut string: String = self.data.iter().collect();
        string.replace_range(self.gap_start..self.gap_end, "");
        string
    }

    pub fn read_from_file(&mut self, path: &str) -> io::Result<()> {
        let contents = fs::read_to_string(path)?;
        for i in contents.chars() {
            self.insert_char(i);
        }
        Ok(())
    }

    pub fn write_to_file(&self, path: &str) -> io::Result<()> {
        let write_data = self.data_to_string();
        fs::write(path, write_data)?;
        Ok(())
    }
}
use std::iter::Iterator;
use std::str::Chars;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bit {
    One,
    Zero,
}

#[derive(Debug)]
pub struct Biterator<'a> {
    pub curr: Option<u32>,
    pub iter: Chars<'a>,
    bit_index: usize,
    pub bits_processed: usize,
}

impl Biterator<'_> {
    pub fn parse(&mut self, num_bits: usize) -> Option<u32> {
        let mut num_builder = NumBuilder::new();
        for _ in 0..num_bits {
            num_builder.push(self.next()?);
        }
        Some(num_builder.build())
    }
}

impl<'a> From<&'a str> for Biterator<'a> {
    fn from(value: &'a str) -> Self {
        let mut chars = value.chars();
        let curr = chars.next();
        Self {
            curr: curr.map(|curr| curr.to_digit(16).unwrap()),
            bit_index: 0,
            iter: chars,
            bits_processed: 0,
        }
    }
}

impl Iterator for Biterator<'_> {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        self.bits_processed += 1;
        match self.curr {
            None => None,
            Some(x) => {
                let result = if x & 0b1000 != 0 {
                    self.curr = self.curr.map(|curr| curr << 1);
                    Some(Bit::One)
                } else {
                    self.curr = self.curr.map(|curr| curr << 1);
                    Some(Bit::Zero)
                };

                self.bit_index += 1;

                if self.bit_index == 4 {
                    self.bit_index = 0;
                    self.curr = self.iter.next().map(|curr| curr.to_digit(16).unwrap());
                }

                result
            }
        }
    }
}

pub struct NumBuilder {
    data: u32,
}

impl NumBuilder {
    pub fn new() -> Self {
        Self { data: 0 }
    }
    pub fn push(&mut self, bit: Bit) {
        self.data <<= 1;
        if let Bit::One = bit {
            self.data |= 1;
        }
    }

    pub fn build(self) -> u32 {
        self.data
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_biterator() {
        use super::Bit::*;
        use super::*;

        let biterator = Biterator::from("ab40f");

        let mut expected = vec![One, Zero, One, Zero];
        expected.extend(vec![One, Zero, One, One]);
        expected.extend(vec![Zero, One, Zero, Zero]);
        expected.extend(vec![Zero, Zero, Zero, Zero]);
        expected.extend(vec![One, One, One, One]);

        assert_eq!(biterator.collect::<Vec<Bit>>(), expected);
    }
}

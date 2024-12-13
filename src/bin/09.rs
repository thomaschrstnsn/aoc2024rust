use std::{iter, ops::Range};

advent_of_code::solution!(9);

fn parse_char(c: char) -> usize {
    c.to_digit(10).map(|d| d as usize).expect("is digit")
}

fn parse(input: &str) -> impl Iterator<Item = usize> + use<'_> {
    input.trim().chars().map(parse_char)
}

struct Disk(Vec<Option<usize>>);

impl Disk {
    fn from_input(input: &[usize]) -> Self {
        let total_blocks: usize = input.iter().sum();

        let mut disk: Vec<Option<usize>> = Vec::with_capacity(total_blocks);

        // explode to disk block repr
        for (index, count_blocks) in input.iter().enumerate() {
            let is_free = index % 2 != 0;
            let block = if is_free { None } else { Some(index / 2) };
            iter::repeat(block)
                .take(*count_blocks)
                .for_each(|_| disk.push(block));
        }
        Self(disk)
    }

    fn first_free_last_occupied(&self) -> (usize, usize) {
        let free = self
            .0
            .iter()
            .position(|block| block.is_none())
            .expect("should have a free block");
        let used = self
            .0
            .iter()
            .rposition(|block| block.is_some())
            .expect("should have a used block");

        (free, used)
    }

    fn checksum(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(index, block)| Some((*block)? * index))
            .sum()
    }

    fn free_sections(&self) -> impl Iterator<Item = Range<usize>> + use<'_> {
        DiskIter {
            disk: self,
            index: 0,
            typ: DiskIterType::Free,
        }
    }

    fn used_sections(&self) -> impl Iterator<Item = Range<usize>> + use<'_> {
        DiskIter {
            disk: self,
            index: 0,
            typ: DiskIterType::Files,
        }
    }
}

enum DiskIterType {
    Free,
    Files,
}

struct DiskIter<'a> {
    disk: &'a Disk,
    index: usize,
    typ: DiskIterType,
}

impl<'a> Iterator for DiskIter<'a> {
    type Item = Range<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        let pred = match self.typ {
            DiskIterType::Free => |x: Option<usize>| x.is_none(),
            DiskIterType::Files => |x: Option<usize>| x.is_some(),
        };

        let mut start = None;
        let mut end = None;
        let mut value: Option<Option<usize>> = None;
        for (index, block) in self.disk.0.iter().enumerate().skip(self.index) {
            if start.is_none() && pred(*block) {
                start = Some(index);
                value = Some(*block);
                continue;
            }
            if start.is_some()
                && end.is_none()
                && (!pred(*block) || *block != value.expect("value is set"))
            {
                end = Some(index);
                break;
            }
        }

        let start = start?;
        let end_of_disk = self.disk.0.len();
        let end = end.unwrap_or(end_of_disk);

        self.index = end;

        Some(start..end)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let parsed: Vec<_> = parse(input).collect();

    let mut disk = Disk::from_input(&parsed);

    // defrag
    let (mut free, mut used) = disk.first_free_last_occupied();
    while free < used {
        disk.0.swap(free, used);
        (free, used) = disk.first_free_last_occupied();
    }

    Some(disk.checksum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let parsed: Vec<_> = parse(input).collect();

    let mut disk = Disk::from_input(&parsed);

    // defrag
    let files: Vec<Range<usize>> = disk.used_sections().collect();
    for file in files.into_iter().rev() {
        let required_size = file.len();
        let free = disk
            .free_sections()
            .filter(|free| free.len() >= required_size)
            .find(|free| free.start < file.start);
        if let Some(free) = free {
            for (from, to) in (free).zip(file) {
                disk.0.swap(from, to);
            }
        }
    }

    Some(disk.checksum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}

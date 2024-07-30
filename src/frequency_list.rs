#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrequencyList {
    frequencies: [u32; 48],
    number_frequencies: u8,
}

impl FrequencyList {
    pub fn new(frequencies: &[u32], number: u8) -> FrequencyList {
        let mut freq_list = FrequencyList {
            ..Default::default()
        };

        freq_list.frequencies[0..(number as usize)]
            .copy_from_slice(&frequencies[0..(number as usize)]);
        freq_list.number_frequencies = number;
        freq_list
    }

    pub fn number(&self) -> u8 {
        self.number_frequencies
    }

    pub fn frequencies(&self) -> &[u32; 48] {
        &self.frequencies
    }
}

// impl From<[u32; 48]> for FrequencyList {
//     fn from(value: [u32; 48]) -> Self {
//         Self(value)
//     }
// }

impl Into<[u32; 48]> for FrequencyList {
    fn into(self) -> [u32; 48] {
        self.frequencies
    }
}

// impl AsRef<[u32; 48]> for FrequencyList {
//     fn as_ref(&self) -> &[u32; 48] {
//         &self.frequencies
//     }
// }

impl Default for FrequencyList {
    fn default() -> Self {
        Self {
            frequencies: [0u32; 48],
            number_frequencies: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_into() {
        let raw_data = [
            1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7,
            8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8,
        ];

        let frequency_list = FrequencyList::new(&raw_data, 48);

        let data: [u32; 48] = frequency_list.into();

        assert_eq!(data, raw_data);
    }

    #[test]
    fn test_number() {
        let raw_data = [
            1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7,
            8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8,
        ];

        let frequency_list = FrequencyList::new(&raw_data, 5);

        assert_eq!(frequency_list.number(), 5);

        let mut expected_frequencies = [0; 48];
        expected_frequencies[0..5].copy_from_slice(&[1, 2, 3, 4, 5]);

        let frequencies: [u32; 48] = frequency_list.into();
        assert_eq!(frequencies, expected_frequencies);
    }

    #[test]
    fn test_frequencies() {
        let raw_data = [
            1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7,
            8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8,
        ];

        let list = FrequencyList::new(&raw_data, 48);

        assert_eq!(&raw_data, list.frequencies());

        let list = FrequencyList::new(&raw_data, 5);

        let mut expected_list = [0u32; 48];
        expected_list[0..5].copy_from_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(&expected_list, list.frequencies());
    }
}

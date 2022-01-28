use bit_field::BitField;

pub struct WiegandFormat {
    pub parity_even: usize,
    pub parity_odd: usize,
    pub facility_code: (usize, usize),
    pub card_number: (usize, usize),
}

#[derive(Debug)]
pub enum WiegandError {
    InvalidParity,
    InvalidRange,
}

impl WiegandFormat {
    /// decode a (facility_code, card_number) tuple from an integer sourced from an RFID scan
    pub fn decode(self, i: u32) -> Result<(u8, u16), WiegandError> {
        let facility_code = i.get_bits(self.facility_code.0..self.facility_code.1) as u8;
        let card_number = i.get_bits(self.card_number.0..self.card_number.1) as u16;

        Ok((facility_code, card_number))
    }

    /// assumes `i` is not padded
    pub fn to_string(self, i: u32) -> String {
        let parity_bit_odd = parity_sum(i, self.card_number) % 2 == 0;
        let parity_bit_even = parity_sum(i, self.facility_code) % 2 == 1;

        format!(
            "{:b}{:b}{:b}",
            parity_bit_even as u8, i, parity_bit_odd as u8
        )
    }
}

/// sum of bits set in integer, i
fn parity_sum(i: u32, range: (usize, usize)) -> u8 {
    let mut sum = 0;

    for bit_index in range.0..=range.1 {
        let bit_set = i & (1 << bit_index) != 0;
        sum += if bit_set { 1 } else { 0 };
    }

    sum % 2
}

#[cfg(test)]
mod tests {
    use super::WiegandFormat;

    #[test]
    fn standard_format() {
        let standard_wiegand = WiegandFormat {
            parity_even: 0,
            parity_odd: 25,
            card_number: (0, 16),
            facility_code: (16, 24),
        };

        let rfid_payload = 5666862u32;

        assert_eq!(standard_wiegand.decode(rfid_payload).unwrap(), (86u8, 30766u16));
    }
}

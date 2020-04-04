use std::collections::BTreeMap;

use failure::Error;

use crate::{error::CodingError, helper::generate_dest_base_encoder};

#[derive(Debug, Default)]
pub struct ArithmeticCoding {
    probability_interval: BTreeMap<char, (f64, f64)>,
}

impl ArithmeticCoding {
    pub fn new(distribution: &BTreeMap<char, f64>) -> Self {
        Self {
            probability_interval: Self::interval_from_distribution(distribution),
        }
    }

    fn interval_from_distribution(
        distribution: &BTreeMap<char, f64>,
    ) -> BTreeMap<char, (f64, f64)> {
        let mut interval = (0_f64, 0_f64);

        distribution
            .iter()
            .map(|(k, x)| {
                interval.0 = interval.1;
                interval.1 += x;
                (*k, interval)
            })
            .collect()
    }

    pub fn reset(&mut self, distribution: &BTreeMap<char, f64>) {
        self.probability_interval = Self::interval_from_distribution(distribution);
    }

    pub fn decimal_to_nbased_float(mut origin: f64, base: u32, length: u64) -> String {
        let mut coding_result = base as u64;

        for _ in 0..length {
            origin *= base as f64;
            let int_part = origin.floor();
            coding_result += int_part as u64;
            coding_result *= base as u64;
            origin -= int_part;
        }
        coding_result += 1;

        let base_n = generate_dest_base_encoder(base);
        base_n.gen(coding_result).chars().skip(1).collect()
    }

    pub fn encode(&self, buffer: &str, base: u32) -> Result<String, Error> {
        let mut chars = buffer.chars();
        let first_letter = chars.next().ok_or(CodingError::CodeBookTooShort)?;
        let mut interval = *self
            .probability_interval
            .get(&first_letter)
            .ok_or(CodingError::NoSuchLetterError(first_letter))?;
        let mut spacing = interval.1 - interval.0;

        for c in chars {
            let (p_lower, p_upper) = self
                .probability_interval
                .get(&c)
                .ok_or(CodingError::NoSuchLetterError(c))?;
            interval.1 = interval.0 + p_upper * spacing;
            interval.0 += p_lower * spacing;
            spacing = interval.1 - interval.0;
        }

        let coding_length = (-spacing.log(base as f64)).ceil() as u64;
        let coding = Self::decimal_to_nbased_float(interval.0, base, coding_length);

        Ok(coding)
    }

    pub fn decode(&self, coded: &str, base: u32) -> Result<String, Error> {
        let min_spacing = (1_f64 / base as f64).powf(coded.len() as f64 - 2_f64);
        let coded = Self::string_to_10_float(coded, base);

        let mut decoded = String::new();
        let mut interval = (0_f64, 0_f64);
        let mut spacing = 1_f64;
        while spacing > min_spacing {
            for (&c, &(lower, upper)) in self.probability_interval.iter() {
                let new_lower = interval.0 + spacing * lower;
                let new_upper = interval.0 + spacing * upper;

                if coded >= new_lower && coded <= new_upper {
                    decoded.push(c);

                    spacing = new_upper - new_lower;
                    interval = (new_lower, new_upper);
                    break;
                }
            }
        }

        Ok(decoded)
    }

    fn string_to_10_float(coded: &str, base: u32) -> f64 {
        coded
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .enumerate()
            .map(|(i, x)| (x as f64) * (1_f64 / base as f64).powf((i + 1) as f64))
            .sum()
    }
}

pub struct DCT {
    constants: Vec<Vec<f32>>,
    multiplier: f32,
}

impl DCT {
    pub fn new(block_size: usize) -> Self {
        let mut size: usize = block_size;
        let mut constants: Vec<Vec<f32>> = Vec::new();
        while size >= 2 {
            size /= 2;
            let mut temp: Vec<f32> = vec![0_f32; size];
            for i in 0..size {
                temp[i] = 0.5_f32
                    / (((i as f32) + 0.5_f32) * std::f32::consts::PI / ((size as f32) * 2_f32))
                        .cos();
            }
            constants.push(temp);
        }
        Self {
            constants,
            multiplier: 0.5_f32.powi(block_size.ilog2() as i32 - 1),
        }
    }

    pub fn dct(&self, data: &mut [f32], temp: &mut [f32]) {
        self.forward_dct(data, temp, 0);
        data[0] = data[0] * 0.5_f32;
        for e in data {
            *e = *e * self.multiplier;
        }
    }

    pub fn idct(&self, data: &mut [f32], temp: &mut [f32]) {
        self.inverse_dct(data, temp, 0);
    }

    fn forward_dct(&self, data: &mut [f32], temp: &mut [f32], depth: usize) {
        let len: usize = data.len();
        let half_len: usize = len / 2;

        for i in 0..half_len {
            let x: f32 = data[i];
            let y: f32 = data[len - i - 1];
            temp[i] = x + y;
            temp[i + half_len] = (x - y) * self.constants[depth][i];
        }

        if len != 2 {
            self.forward_dct(&mut temp[0..half_len], &mut data[0..half_len], depth + 1);
            self.forward_dct(&mut temp[half_len..len], &mut data[0..half_len], depth + 1);
        }

        for i in 0..half_len - 1 {
            data[i * 2] = temp[i];
            data[i * 2 + 1] = temp[i + half_len] + temp[i + half_len + 1];
        }
        data[len - 2] = temp[half_len - 1];
        data[len - 1] = temp[len - 1];
    }

    fn inverse_dct(&self, data: &mut [f32], temp: &mut [f32], depth: usize) {
        let len: usize = data.len();
        let half_len: usize = len / 2;

        temp[0] = data[0];
        temp[half_len] = data[1];

        for i in 1..half_len {
            temp[i] = data[i * 2];
            temp[i + half_len] = data[2 * i - 1] + data[2 * i + 1];
        }

        if len != 2 {
            self.inverse_dct(&mut temp[0..half_len], &mut data[0..half_len], depth + 1);
            self.inverse_dct(&mut temp[half_len..len], &mut data[0..half_len], depth + 1);
        }

        for i in 0..half_len {
            let x = temp[i];
            let y = temp[i + half_len] * self.constants[depth][i];
            data[i] = x + y;
            data[len - 1 - i] = x - y;
        }
    }
}

#[cfg(test)]
mod test_dct {

    use super::DCT;

    fn get_test_case() -> [f32; 8] {
        [
            500_f32, 100_f32, -150_f32, 1500_f32, 1000_f32, 5000_f32, -1000_f32, 400_f32,
        ]
    }
    #[test]
    fn test_dct_idct() {
        let data_orig: [f32; 8] = get_test_case();
        let mut data = data_orig.clone();
        let mut temp = [-100000000000_f32; 8];

        let dct: DCT = DCT::new(8);

        println!(
            "{}, {}",
            std::f32::consts::FRAC_1_SQRT_2,
            dct.constants[2][0]
        );

        dct.dct(data.as_mut_slice(), temp.as_mut_slice());
        eprintln!("{data:?}\n{temp:?}\n{data_orig:?}");
        dct.idct(data.as_mut_slice(), temp.as_mut_slice());
        eprintln!("{data:?}\n{temp:?}\n{data_orig:?}");
        for i in 0..8 {
            assert!((data[i] - data_orig[i]) < 0.01_f32);
        }
    }
}

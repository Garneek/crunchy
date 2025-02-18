mod dct;

pub struct MDCT {
    dct: dct::DCT,
    block_size: usize,

    dct_buffer: Vec<f32>,
    idct_buffer: Vec<f32>,
    window: Vec<f32>,
    temp_buffer: Vec<f32>,
}

impl MDCT {
    pub fn new(block_size: usize) -> Self {
        Self {
            dct: dct::DCT::new(block_size * 2),
            block_size,

            dct_buffer: vec![0_f32; block_size * 2],
            idct_buffer: vec![0_f32; block_size],
            window: (0..block_size * 2)
                .into_iter()
                .map(|i| {
                    (std::f32::consts::PI * i as f32 / ((2 * block_size + 1) as f32))
                        .sin()
                        .powi(2)
                })
                .collect(),
            temp_buffer: vec![0_f32; block_size * 2],
        }
    }

    // Processes sample block into some output.
    // Sample block needs to be a power of 2.
    // Output block should be 2 times larger then input block
    pub fn mdct(&mut self, block: &[f32], output_target: &mut [f32]) {
        self.dct_buffer[self.block_size..self.block_size * 2]
            .copy_from_slice(&block[0..self.block_size]);

        output_target.copy_from_slice(&self.dct_buffer);

        self.dct.dct(output_target, self.temp_buffer.as_mut_slice());

        let (first, second) = self.dct_buffer.split_at_mut(self.block_size);
        first.copy_from_slice(second);
    }

    // Processes dct data into block of samples.
    // Dct block needs to be a power of 2.
    // Output block should be 2 times smaller then input block
    pub fn imdct(&mut self, dct_block: &mut [f32], output_block: &mut [f32]) {
        self.dct.idct(dct_block, self.temp_buffer.as_mut_slice());

        for i in 0..self.block_size {
            output_block[i] = dct_block[i].mul_add(self.window[i], self.idct_buffer[i]);
        }

        for i in self.block_size..self.block_size * 2 {
            self.idct_buffer[i - self.block_size] = dct_block[i] * self.window[i];
        }
    }
}

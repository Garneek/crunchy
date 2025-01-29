mod dct;

pub const BLOCK_SIZE: usize = 64;

pub struct MDCT {
    dct: dct::DCT,
    dct_buffer: [f32; BLOCK_SIZE * 2],
    idct_buffer: [f32; BLOCK_SIZE],
    window: [f32; BLOCK_SIZE * 2],
    temp_buffer: [f32; BLOCK_SIZE * 2],
}

impl MDCT {
    pub fn new() -> Self {
        let mut temp = Self {
            dct: dct::DCT::new(BLOCK_SIZE * 2),
            dct_buffer: [0_f32; BLOCK_SIZE * 2],
            idct_buffer: [0_f32; BLOCK_SIZE],
            window: [0_f32; BLOCK_SIZE * 2],
            temp_buffer: [0_f32; BLOCK_SIZE * 2],
        };
        for i in 0..BLOCK_SIZE * 2 {
            temp.window[i] = (std::f32::consts::PI * i as f32 / ((2 * BLOCK_SIZE + 1) as f32))
                .sin()
                .powi(2);
        }
        temp
    }

    // Processes sample block into some output.
    // Sample block needs to be a power of 2.
    // Output block should be 2 times larger then input block
    pub fn mdct(&mut self, block: &mut [f32], output_target: &mut [f32]) {
        for i in BLOCK_SIZE..BLOCK_SIZE * 2 {
            self.dct_buffer[i] = block[i - BLOCK_SIZE];
        }

        for i in 0..BLOCK_SIZE * 2 {
            output_target[i] = self.dct_buffer[i];
        }

        self.dct.dct(output_target, self.temp_buffer.as_mut_slice());
        for i in 0..BLOCK_SIZE {
            self.dct_buffer[i] = self.dct_buffer[i + BLOCK_SIZE];
        }
    }

    // Processes dct data into block of samples.
    // Dct block needs to be a power of 2.
    // Output block should be 2 times smaller then input block
    pub fn imdct(&mut self, dct_block: &mut [f32], output_block: &mut [f32]) {
        self.dct.idct(dct_block, self.temp_buffer.as_mut_slice());
        for i in 0..BLOCK_SIZE {
            output_block[i] = dct_block[i].mul_add(self.window[i], self.idct_buffer[i]);
        }

        for i in BLOCK_SIZE..BLOCK_SIZE * 2 {
            self.idct_buffer[i - BLOCK_SIZE] = dct_block[i] * self.window[i];
        }
    }
}

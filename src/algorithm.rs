use crate::CrunchyParams;
use nih_plug::buffer::Buffer;
use nih_plug::plugin::ProcessStatus;
use std::sync::Arc;

mod mdct;

use mdct::BLOCK_SIZE;

const CHANNELS: usize = 2;
const CRUNCH_MAX: f32 = 0.05_f32;

pub struct SingleChannelProcessor {
    mdct: mdct::MDCT,

    dct_buffer: [f32; BLOCK_SIZE * 2],

    delay_buffer: [f32; BLOCK_SIZE],
    mix_buffer: [f32; BLOCK_SIZE],
}

impl SingleChannelProcessor {
    pub fn new() -> Self {
        Self {
            mdct: mdct::MDCT::new(),
            dct_buffer: [0_f32; BLOCK_SIZE * 2],
            mix_buffer: [0_f32; BLOCK_SIZE],
            delay_buffer: [0_f32; BLOCK_SIZE],
        }
    }

    fn process_block(&mut self, block: &mut [f32], params_block: &ParamsBlock) -> ProcessStatus {
        let len: usize = block.len();
        if len != BLOCK_SIZE {
            ProcessStatus::Error("Block size not divisible by 64. Fix later")
        } else {
            // Clone block for mix
            self.delay_buffer.clone_from_slice(block);
            // Apply drive
            for i in 0..len {
                block[i] *= params_block.drive[i];
            }

            self.mdct.mdct(block, self.dct_buffer.as_mut_slice());

            let crunch = (params_block.crunch[BLOCK_SIZE / 2] / 100_f32).powi(2);

            if crunch != 0_f32 {
                let crunch_clamp = 1.01_f32 - crunch;
                let crunch_gain = 1_f32 / crunch_clamp.sqrt(); // RETHINK IF GAIN IS WELL MADE

                for i in 0..BLOCK_SIZE * 2 {
                    self.dct_buffer[i] = self.dct_buffer[i]
                        .clamp(-CRUNCH_MAX * crunch_clamp, CRUNCH_MAX * crunch_clamp);
                    self.dct_buffer[i] *= crunch_gain;
                }
            }

            let crush = params_block.crush[BLOCK_SIZE / 2] / 100_f32;

            if crush != 0_f32 {
                let crush_multiplier = (1_f32 - crush) * 256_f32 + 16_f32;
                for i in 0..BLOCK_SIZE * 2 {
                    self.dct_buffer[i] =
                        (self.dct_buffer[i] * crush_multiplier).round() / crush_multiplier;
                }
            }

            self.mdct.imdct(self.dct_buffer.as_mut_slice(), block);

            for i in 0..len {
                // Apply mix and gain
                block[i] = block[i].mul_add(
                    params_block.mix[i] / 100_f32,
                    self.mix_buffer[i] * (1_f32 - params_block.mix[i] / 100_f32),
                ) * params_block.gain[i];
            }

            self.mix_buffer.clone_from(&self.delay_buffer);

            ProcessStatus::Normal
        }
    }
}

pub struct DCTCrush {
    channels: [SingleChannelProcessor; CHANNELS],
    params_block: ParamsBlock,
    params: Arc<CrunchyParams>,
}

impl DCTCrush {
    pub fn new(params: Arc<CrunchyParams>) -> Self {
        let params_block = ParamsBlock::default();
        Self {
            channels: core::array::from_fn(|_| SingleChannelProcessor::new()),
            params_block,
            params,
        }
    }

    pub fn process(&mut self, buffer: &mut Buffer) -> ProcessStatus {
        for block in buffer.iter_blocks(BLOCK_SIZE) {
            let mut block_channels = block.1.into_iter();

            self.params_block
                .from_params(self.params.clone(), BLOCK_SIZE);

            match self.channels[0].process_block(
                match block_channels.next() {
                    Some(v) => v,
                    None => {
                        return ProcessStatus::Error("Not enough channels (less then one)");
                    }
                },
                &self.params_block,
            ) {
                ProcessStatus::Error(e) => return ProcessStatus::Error(e),
                _ => {}
            }

            if let Some(v) = block_channels.next() {
                match self.channels[1].process_block(v, &self.params_block) {
                    ProcessStatus::Error(e) => return ProcessStatus::Error(e),
                    _ => {}
                }
            }
        }
        ProcessStatus::Normal
    }
}

pub struct ParamsBlock {
    drive: [f32; BLOCK_SIZE],
    crunch: [f32; BLOCK_SIZE],
    crush: [f32; BLOCK_SIZE],
    mix: [f32; BLOCK_SIZE],
    gain: [f32; BLOCK_SIZE],
}

impl Default for ParamsBlock {
    fn default() -> Self {
        Self {
            drive: [0_f32; BLOCK_SIZE],
            crunch: [0_f32; BLOCK_SIZE],
            crush: [0_f32; BLOCK_SIZE],
            mix: [0_f32; BLOCK_SIZE],
            gain: [0_f32; BLOCK_SIZE],
        }
    }
}

impl ParamsBlock {
    fn from_params(&mut self, params: Arc<CrunchyParams>, len: usize) {
        params
            .drive
            .smoothed
            .next_block(self.drive.as_mut_slice(), len);
        params
            .crunch
            .smoothed
            .next_block(self.crunch.as_mut_slice(), len);
        params
            .crush
            .smoothed
            .next_block(self.crush.as_mut_slice(), len);
        params.mix.smoothed.next_block(self.mix.as_mut_slice(), len);
        params
            .gain
            .smoothed
            .next_block(self.gain.as_mut_slice(), len);
    }
}

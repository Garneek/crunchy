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

    fn process_block(
        &mut self,
        block: &[f32],
        output: &mut [f32],
        params_block: &ParamsBlock,
    ) -> ProcessStatus {
        let len: usize = block.len();
        // Clone block for mix
        self.delay_buffer.copy_from_slice(block);
        // Apply drive
        for i in 0..len {
            output[i] = block[i] * params_block.drive[i];
        }

        self.mdct.mdct(output, self.dct_buffer.as_mut_slice());

        let crunch = params_block.crunch[BLOCK_SIZE / 2].powi(2);

        if crunch != 0_f32 {
            let crunch_clamp = 1.01_f32 - crunch;
            let crunch_gain = 1_f32 / crunch_clamp.sqrt(); // RETHINK IF GAIN IS WELL MADE

            for i in 0..BLOCK_SIZE * 2 {
                self.dct_buffer[i] =
                    self.dct_buffer[i].clamp(-CRUNCH_MAX * crunch_clamp, CRUNCH_MAX * crunch_clamp);
                self.dct_buffer[i] *= crunch_gain;
            }
        }

        let crush = params_block.crush[BLOCK_SIZE / 2];

        if crush != 0_f32 {
            let crush_multiplier = (1_f32 - crush) * 256_f32 + 16_f32;
            for i in 0..BLOCK_SIZE * 2 {
                self.dct_buffer[i] =
                    (self.dct_buffer[i] * crush_multiplier).round() / crush_multiplier;
            }
        }

        self.mdct.imdct(self.dct_buffer.as_mut_slice(), output);

        for i in 0..len {
            // Apply mix and gain
            output[i] = output[i].mul_add(
                params_block.mix[i],
                self.mix_buffer[i] * (1_f32 - params_block.mix[i]),
            ) * params_block.gain[i];
        }

        self.mix_buffer.copy_from_slice(&self.delay_buffer);

        ProcessStatus::Normal
    }
}

pub struct DCTCrush {
    channel_processor: [SingleChannelProcessor; CHANNELS],

    overflow: usize,
    temp: [f32; BLOCK_SIZE],
    buffer: [[f32; BLOCK_SIZE]; CHANNELS],

    params_block: ParamsBlock,
}

impl DCTCrush {
    pub fn new(params: Arc<CrunchyParams>) -> Self {
        Self {
            channel_processor: core::array::from_fn(|i| SingleChannelProcessor::new()),
            overflow: BLOCK_SIZE,
            temp: [0_f32; BLOCK_SIZE],
            buffer: [[0_f32; BLOCK_SIZE]; CHANNELS],

            params_block: ParamsBlock::new(params),
        }
    }

    pub fn process(&mut self, buffer: &mut Buffer) -> ProcessStatus {
        let samples = buffer.samples();
        let channels = buffer.channels().min(CHANNELS);
        let slice = buffer.as_slice();

        if slice.len() == 0 {
            return ProcessStatus::Error("No channels");
        }

        let mut index = BLOCK_SIZE - self.overflow;
        if index != 0 {
            for channel in 0..channels {
                self.temp[0..self.overflow]
                    .copy_from_slice(&self.buffer[channel][0..self.overflow]);

                self.temp[self.overflow..BLOCK_SIZE].copy_from_slice(&slice[channel][0..index]);
                slice[channel][0..index]
                    .copy_from_slice(&self.buffer[channel][self.overflow..BLOCK_SIZE]);

                self.params_block.from_params(BLOCK_SIZE);
                match self.channel_processor[channel].process_block(
                    &self.temp,
                    &mut self.buffer[channel],
                    &self.params_block,
                ) {
                    ProcessStatus::Error(e) => return ProcessStatus::Error(e),
                    _ => {}
                };
            }
            self.overflow = BLOCK_SIZE;
        }

        let index_static = index;
        for _ in 0..(samples - index_static) / BLOCK_SIZE {
            for channel in 0..channels {
                self.temp
                    .copy_from_slice(&slice[channel][index..index + BLOCK_SIZE]);
                slice[channel][index..index + BLOCK_SIZE]
                    .copy_from_slice(&self.buffer[channel][0..BLOCK_SIZE]);

                self.params_block.from_params(BLOCK_SIZE);
                match self.channel_processor[channel].process_block(
                    &self.temp,
                    &mut self.buffer[channel],
                    &self.params_block,
                ) {
                    ProcessStatus::Error(e) => return ProcessStatus::Error(e),
                    _ => {}
                };
            }
            index += BLOCK_SIZE;
        }

        if index != samples {
            self.overflow = samples - index;

            for channel in 0..channels {
                let mut t;
                for i in 0..self.overflow {
                    t = slice[channel][i + index];
                    slice[channel][i + index] = self.buffer[channel][i];
                    self.buffer[channel][i] = t;
                }
            }
        }

        ProcessStatus::Normal
    }
}

pub struct ParamsBlock {
    params: Arc<CrunchyParams>,
    drive: [f32; BLOCK_SIZE],
    crunch: [f32; BLOCK_SIZE],
    crush: [f32; BLOCK_SIZE],
    mix: [f32; BLOCK_SIZE],
    gain: [f32; BLOCK_SIZE],
}

impl ParamsBlock {
    fn new(params: Arc<CrunchyParams>) -> Self {
        Self {
            params,
            drive: [0_f32; BLOCK_SIZE],
            crunch: [0_f32; BLOCK_SIZE],
            crush: [0_f32; BLOCK_SIZE],
            mix: [0_f32; BLOCK_SIZE],
            gain: [0_f32; BLOCK_SIZE],
        }
    }

    fn from_params(&mut self, len: usize) {
        self.params
            .drive
            .smoothed
            .next_block(self.drive.as_mut_slice(), len);
        self.params
            .crunch
            .smoothed
            .next_block(self.crunch.as_mut_slice(), len);
        self.params
            .crush
            .smoothed
            .next_block(self.crush.as_mut_slice(), len);
        self.params
            .mix
            .smoothed
            .next_block(self.mix.as_mut_slice(), len);
        self.params
            .gain
            .smoothed
            .next_block(self.gain.as_mut_slice(), len);
    }
}

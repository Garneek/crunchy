use crate::CrunchyParams;
use std::sync::Arc;

use nih_plug::plugin::ProcessStatus;
use plugin_utils::dsp_utils::ParamsBlock;
use plugin_utils::dsp_utils::SingleChannelProcessor;
use plugin_utils::dsp_utils::MDCT;

const CRUNCH_MAX: f32 = 0.05_f32;

pub struct CrunchySingleChannelProcessor {
    mdct: MDCT,
    block_size: usize,

    dct_buffer: Vec<f32>,

    delay_buffer: Vec<f32>,
    mix_buffer: Vec<f32>,
}

impl SingleChannelProcessor for CrunchySingleChannelProcessor {
    type ParamsBlock = CrunchyParamsBlock;

    fn new(block_size: usize) -> Self {
        Self {
            mdct: MDCT::new(block_size),
            block_size,
            dct_buffer: vec![0_f32; block_size * 2],
            mix_buffer: vec![0_f32; block_size],
            delay_buffer: vec![0_f32; block_size],
        }
    }

    fn process(
        &mut self,
        block: &[f32],
        output: &mut [f32],
        params_block: &Self::ParamsBlock,
    ) -> nih_plug::prelude::ProcessStatus {
        let len: usize = block.len();
        // Clone block for mix
        self.delay_buffer.copy_from_slice(block);
        // Apply drive
        for i in 0..len {
            output[i] = block[i] * params_block.drive[i];
        }

        self.mdct.mdct(output, self.dct_buffer.as_mut_slice());

        let crunch = params_block.crunch[self.block_size / 2].powi(2);

        if crunch != 0_f32 {
            let crunch_clamp = 1.01_f32 - crunch;
            let crunch_gain = 1_f32 / crunch_clamp.sqrt(); // RETHINK IF GAIN IS WELL MADE

            for i in 0..self.block_size * 2 {
                self.dct_buffer[i] =
                    self.dct_buffer[i].clamp(-CRUNCH_MAX * crunch_clamp, CRUNCH_MAX * crunch_clamp);
                self.dct_buffer[i] *= crunch_gain;
            }
        }

        let crush = params_block.crush[self.block_size / 2];

        if crush != 0_f32 {
            let crush_multiplier = (1_f32 - crush) * 256_f32 + 16_f32;
            for i in 0..self.block_size * 2 {
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

pub struct CrunchyParamsBlock {
    params: Arc<CrunchyParams>,
    block_size: usize,

    drive: Vec<f32>,
    crunch: Vec<f32>,
    crush: Vec<f32>,
    mix: Vec<f32>,
    gain: Vec<f32>,
}

impl ParamsBlock for CrunchyParamsBlock {
    type Params = CrunchyParams;
    fn new(params: Arc<Self::Params>, block_size: usize) -> Self {
        Self {
            params,
            block_size,
            drive: vec![0_f32; block_size],
            crunch: vec![0_f32; block_size],
            crush: vec![0_f32; block_size],
            mix: vec![0_f32; block_size],
            gain: vec![0_f32; block_size],
        }
    }

    fn from_params(&mut self) {
        self.params
            .drive
            .smoothed
            .next_block(self.drive.as_mut_slice(), self.block_size);
        self.params
            .crunch
            .smoothed
            .next_block(self.crunch.as_mut_slice(), self.block_size);
        self.params
            .crush
            .smoothed
            .next_block(self.crush.as_mut_slice(), self.block_size);
        self.params
            .mix
            .smoothed
            .next_block(self.mix.as_mut_slice(), self.block_size);
        self.params
            .gain
            .smoothed
            .next_block(self.gain.as_mut_slice(), self.block_size);
    }
}

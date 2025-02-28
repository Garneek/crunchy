use crate::CrunchyParams;
use std::sync::Arc;

use nih_plug::plugin::ProcessStatus;

use plugin_utils::dsp_utils::numerical_functions::quartic;
use plugin_utils::dsp_utils::rescale_normalized_value;
use plugin_utils::dsp_utils::rescalers::ln;
use plugin_utils::dsp_utils::rescalers::ln_reversed_unscaled_default;
use plugin_utils::dsp_utils::ParamsBlock;
use plugin_utils::dsp_utils::SingleChannelProcessor;
use plugin_utils::dsp_utils::MDCT;

const CRUSH_RESCALE_MIN: f32 = 0.1_f32;
const CRUSH_RESCALE_MAX: f32 = 0.98_f32;
const CRUSH_GAIN_A: f32 = 0.008_f32;
const CRUSH_GAIN_B: i32 = 35;
const CRUSH_MULTIPLIER_A: f32 = 128_f32;
const CRUSH_MULTIPLIER_B: f32 = 2_f32;

const CRUNCH_GAIN_QUARTIC_A: f32 = -106.38591_f32;
const CRUNCH_GAIN_QUARTIC_B: f32 = 168.68996_f32;
const CRUNCH_GAIN_QUARTIC_C: f32 = -95.357_f32;
const CRUNCH_GAIN_QUARTIC_D: f32 = 12.67889_f32;
const CRUNCH_GAIN_LINEAR_A: f32 = -100_f32;
const CRUNCH_GAIN_LINEAR_B: f32 = 97_f32;
const CRUNCH_MULTIPLIER: f32 = 0.1_f32;
const CRUNCH_CLAMP_A: f32 = -4.99_f32;
const CRUNCH_CLAMP_B: f32 = 5_f32;

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

        let mut compound_gain = 1_f32;

        let crunch = params_block.crunch[self.block_size / 2];
        if crunch != 0_f32 {
            compound_gain = 0.1_f32.powf(
                (quartic(
                    crunch,
                    CRUNCH_GAIN_QUARTIC_A,
                    CRUNCH_GAIN_QUARTIC_B,
                    CRUNCH_GAIN_QUARTIC_C,
                    CRUNCH_GAIN_QUARTIC_D,
                    if crunch > 0.97 {
                        crunch.mul_add(CRUNCH_GAIN_LINEAR_A, CRUNCH_GAIN_LINEAR_B)
                    } else {
                        0_f32
                    },
                )) * 0.05_f32,
            );

            let crunch = ln(crunch, 0.001).sqrt();

            let crunch_clamp = crunch.mul_add(CRUNCH_CLAMP_A, CRUNCH_CLAMP_B);

            for i in 0..self.block_size * 2 {
                self.dct_buffer[i] = self.dct_buffer[i].clamp(
                    -CRUNCH_MULTIPLIER * crunch_clamp,
                    CRUNCH_MULTIPLIER * crunch_clamp,
                );
            }
        }

        let crush = params_block.crush[self.block_size / 2];
        if crush != 0_f32 {
            let crush = rescale_normalized_value(crush, CRUSH_RESCALE_MIN, CRUSH_RESCALE_MAX);

            compound_gain *= if crush > 0.85_f32 {
                (crush + CRUSH_GAIN_A).powi(CRUSH_GAIN_B).exp()
            } else {
                1_f32
            };

            let crush = ln_reversed_unscaled_default(crush);

            let crush_multiplier = crush.mul_add(CRUSH_MULTIPLIER_A, CRUSH_MULTIPLIER_B);

            for i in 0..self.block_size * 2 {
                self.dct_buffer[i] =
                    (self.dct_buffer[i] * crush_multiplier).round() / crush_multiplier;
            }
        }

        self.mdct.imdct(self.dct_buffer.as_mut_slice(), output);

        // Apply gain correction
        if compound_gain != 1_f32 {
            for i in 0..len {
                output[i] *= compound_gain;
            }
        }

        // Apply mix and gain
        for i in 0..len {
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
    pub block_size: usize,

    pub drive: Vec<f32>,
    pub crunch: Vec<f32>,
    pub crush: Vec<f32>,
    pub mix: Vec<f32>,
    pub gain: Vec<f32>,
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

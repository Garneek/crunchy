use std::sync::Arc;

use nih_plug::buffer::Buffer;
use nih_plug::params::Params;
use nih_plug::prelude::ProcessStatus;

mod mdct;
pub use mdct::MDCT;

pub trait SingleChannelProcessor {
    type ParamsBlock: ParamsBlock;
    fn new(block_size: usize) -> Self;
    fn process(
        &mut self,
        block: &[f32],
        output: &mut [f32],
        params_block: &Self::ParamsBlock,
    ) -> ProcessStatus;
}

pub trait ParamsBlock {
    type Params: Params;
    fn new(params: Arc<Self::Params>, block_size: usize) -> Self;
    fn from_params(&mut self);
}

pub struct DspCoreProcessor<SCP: SingleChannelProcessor> {
    channel_processor: Vec<SCP>,

    overflow: usize,
    temp: Vec<f32>,
    buffer: Vec<Vec<f32>>,
    params_block: SCP::ParamsBlock,

    block_size: usize,
    channels: usize,
}

impl<SCP: SingleChannelProcessor> DspCoreProcessor<SCP> {
    pub fn new(
        params: Arc<<<SCP as SingleChannelProcessor>::ParamsBlock as ParamsBlock>::Params>,
        block_size: usize,
        channels: usize,
    ) -> Self {
        Self {
            channel_processor: (0..channels)
                .into_iter()
                .map(|_| SCP::new(block_size))
                .collect(),
            overflow: block_size,
            temp: vec![0_f32; block_size],
            buffer: Vec::from_iter(std::iter::repeat_n(
                Vec::from_iter(std::iter::repeat_n(0_f32, block_size)),
                channels,
            )),

            params_block: SCP::ParamsBlock::new(params, block_size),

            block_size,
            channels,
        }
    }

    pub fn process(&mut self, buffer: &mut Buffer) -> ProcessStatus {
        let samples = buffer.samples();
        let channels = buffer.channels().min(self.channels);
        let slice = buffer.as_slice();

        if slice.len() == 0 {
            return ProcessStatus::Error("No channels");
        }

        let mut index = self.block_size - self.overflow;
        if index != 0 {
            for channel in 0..channels {
                self.temp[0..self.overflow]
                    .copy_from_slice(&self.buffer[channel][0..self.overflow]);

                self.temp[self.overflow..self.block_size]
                    .copy_from_slice(&slice[channel][0..index]);
                slice[channel][0..index]
                    .copy_from_slice(&self.buffer[channel][self.overflow..self.block_size]);

                self.params_block.from_params();
                match self.channel_processor[channel].process(
                    &self.temp,
                    &mut self.buffer[channel],
                    &self.params_block,
                ) {
                    ProcessStatus::Error(e) => return ProcessStatus::Error(e),
                    _ => {}
                };
            }
            self.overflow = self.block_size;
        }

        let index_static = index;
        for _ in 0..(samples - index_static) / self.block_size {
            for channel in 0..channels {
                self.temp
                    .copy_from_slice(&slice[channel][index..index + self.block_size]);
                slice[channel][index..index + self.block_size]
                    .copy_from_slice(&self.buffer[channel][0..self.block_size]);

                self.params_block.from_params();
                match self.channel_processor[channel].process(
                    &self.temp,
                    &mut self.buffer[channel],
                    &self.params_block,
                ) {
                    ProcessStatus::Error(e) => return ProcessStatus::Error(e),
                    _ => {}
                };
            }
            index += self.block_size;
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

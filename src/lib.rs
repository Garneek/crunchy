use nih_plug::prelude::*;
use nih_plug_egui::EguiState;
use std::sync::Arc;

mod editor;

mod dsp;
use dsp::CrunchySingleChannelProcessor;

use dsp_core::DspCoreProcessor;

// TODO
// [ ] - Rethink names of the effects
// [ ] - Test odd block sizes
// [ ] - Properly test ableton, FL, waveform, LMMS, reaper on all platforms
// [ ] - MacOS build

const BLOCK_SIZE: usize = 64;

struct Crunchy {
    params: Arc<CrunchyParams>,
    dsp: Option<DspCoreProcessor<CrunchySingleChannelProcessor>>,
}

impl Default for Crunchy {
    fn default() -> Self {
        let params = Arc::new(CrunchyParams::default());

        Self {
            params: params.clone(),
            dsp: None,
        }
    }
}

#[derive(Params)]
pub struct CrunchyParams {
    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,

    #[id = "drive"]
    pub drive: FloatParam,
    #[id = "crunch"]
    pub crunch: FloatParam,
    #[id = "crush"]
    pub crush: FloatParam,
    #[id = "mix"]
    pub mix: FloatParam,
    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for CrunchyParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            drive: FloatParam::new(
                "Drive",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            crunch: FloatParam::new(
                "Crunch",
                0.2_f32,
                FloatRange::Linear {
                    min: 0_f32,
                    max: 1_f32,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50_f32))
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_percentage(2))
            .with_string_to_value(formatters::s2v_f32_percentage()),

            crush: FloatParam::new(
                "Crush",
                0.2_f32,
                FloatRange::Linear {
                    min: 0_f32,
                    max: 1_f32,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50_f32))
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_percentage(2))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            mix: FloatParam::new(
                "Mix",
                1_f32,
                FloatRange::Linear {
                    min: 0_f32,
                    max: 1_f32,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50_f32))
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_percentage(2))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

impl Plugin for Crunchy {
    const NAME: &'static str = "Crunchy";
    const VENDOR: &'static str = "Garneek";
    const URL: &'static str = "https://github.com/Garneek/crunchy";
    const EMAIL: &'static str = "";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.params.editor_state.clone())
    }

    fn initialize(
        &mut self,
        audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.dsp = Some(DspCoreProcessor::new(
            self.params.clone(),
            BLOCK_SIZE,
            match audio_io_layout.main_input_channels {
                Some(v) => v.get() as usize,
                None => {
                    return false;
                }
            },
        ));
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        if let Some(algo) = &mut self.dsp {
            algo.process(buffer)
        } else {
            ProcessStatus::Error("DSP data not initialized")
        }
    }
}

impl ClapPlugin for Crunchy {
    const CLAP_ID: &'static str = "garneek.crunchy";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("DCT clip/bitcrush");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = Some("https://github.com/Garneek/crunchy");
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Distortion,
        ClapFeature::Glitch,
    ];
}

impl Vst3Plugin for Crunchy {
    const VST3_CLASS_ID: [u8; 16] = *b"garneek.crunchy_";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Distortion];
}

nih_export_clap!(Crunchy);
nih_export_vst3!(Crunchy);

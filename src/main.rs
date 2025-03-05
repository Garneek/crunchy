#[cfg(feature = "test")]
macro_rules! plot_param_mean_peak {
    ($param:ident, $name:expr, $param_type:ident, $zero_params:ident) => {
        plot::<crunchy_plugin::CrunchySingleChannelProcessor>(
            Arc::new(CrunchyParams::default()),
            |params_block, val| params_block.$param = vec![val; params_block.block_size],
            $zero_params,
            &$param_type,
            $name,
            64,
            "herdbound.mp3",
            PlotType::MeanLoudness,
        );
    };
}

#[cfg(feature = "test")]
macro_rules! plot_param_rms {
    ($param:ident, $name:expr, $param_type:ident, $zero_params:ident) => {
        plot::<crunchy_plugin::CrunchySingleChannelProcessor>(
            Arc::new(CrunchyParams::default()),
            |params_block, val| params_block.$param = vec![val; params_block.block_size],
            $zero_params,
            &$param_type,
            $name,
            64,
            "herdbound.mp3",
            PlotType::Rms,
        );
    };
}

#[cfg(feature = "test")]
fn main() {
    use crunchy_plugin::CrunchyParams;
    use crunchy_plugin::CrunchyParamsBlock;
    // use plugin_utils::dsp_utils::benchmark;
    use plugin_utils::dsp_utils::plot;
    use plugin_utils::dsp_utils::PlotParamData;
    use plugin_utils::dsp_utils::PlotType;
    use std::sync::Arc;

    let zero_params = |params_block: &mut CrunchyParamsBlock| {
        params_block.drive = vec![1_f32; params_block.block_size];
        params_block.crunch = vec![0_f32; params_block.block_size];
        params_block.crush = vec![0_f32; params_block.block_size];
        params_block.mix = vec![1_f32; params_block.block_size];
        params_block.gain = vec![1_f32; params_block.block_size];
    };

    let default_params = |params_block: &mut CrunchyParamsBlock| {
        params_block.drive = vec![1_f32; params_block.block_size];
        params_block.crunch = vec![0.2_f32; params_block.block_size];
        params_block.crush = vec![0.2_f32; params_block.block_size];
        params_block.mix = vec![1_f32; params_block.block_size];
        params_block.gain = vec![1_f32; params_block.block_size];
    };

    let db_params = PlotParamData {
        param_min: -30_f32,
        param_max: 30_f32,
        param_db: true,
    };
    let pr_params = PlotParamData {
        param_min: 0_f32,
        param_max: 1_f32,
        param_db: false,
    };
    // plot_param_rms!(crunch, "crunch", pr_params, zero_params);
    // plot_param_rms!(crush, "crush", pr_params, zero_params);

    // benchmark::<crunchy_plugin::CrunchySingleChannelProcessor>(
    //     Arc::new(CrunchyParams::default()),
    //     default_params,
    //     64,
    //     440,
    //     "herdbound.mp3",
    // );
}

#[cfg(not(feature = "test"))]
fn main() {
    println!("For plotting the params enable \"test\" feature");
}

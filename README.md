# crunchy
An audio effect plugin in rust, using nih_plug. Clips and bitcrushes DCT coefficients of the soundwave, resulting in either raw, mostly high-pitched screaming sound, or weird wobbly effect somewhat similiar to reducing bitrate in mp3 files. 
The project is still very much work in progress and might not work on all DAWs.

# Compiling
Run
``cargo xtask bundle crunchy-plugin --release``
in project root
# Known issues
- plugin cannot be resized in most DAWs on Windows
- plugin generates noise in Waveform 13
- the gui is unresponsive
- sliders are weird looking
- in some DAWs the window might be resizable beyond its aspect ratio

# TODO
- improve readability of the text
- design/copy knobs for ParamSlider interface
- rethink the names of parameters as they might be confusing
- restyle the plugin

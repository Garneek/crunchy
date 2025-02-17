# crunchy
An audio effect plugin in rust, using nih_plug. Clips and bitcrushes DCT coefficients of the soundwave, resulting in either raw, mostly high-pitched screaming sound, or weird wobbly effect somewhat similiar to reducing bitrate in mp3 files. 
The project is still very much work in progress and might not work on all DAWs.

# Compiling
Run
``cargo xtask bundle crunchy-plugin --release``
in project root

# Known issues
- plugin generates noise in Waveform 13
- the gui is unresponsive
- the effects cause noticable difference in loudness of the sound
- there is a sudden jump between 0% and 0.5% for both effects
- the knobs do no redraw correctly on small parameter changes

# TODO
- rethink the names of parameters as they might be confusing
- test the plugin on all platforms
- benchmark the following DCT optimisations:
- > Fixed point numbers
- > 10.1016/j.dsp.2008.11.004

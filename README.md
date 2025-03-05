# crunchy
An audio effect plugin in rust, using nih_plug. Clips and bitcrushes DCT coefficients of the soundwave, resulting in either raw, mostly high-pitched screaming sound, or weird wobbly effect somewhat similiar to reducing bitrate in mp3 files. 
The project is still very much work in progress and might not work on all DAWs.

# Compiling
Run
``cargo xtask bundle crunchy-plugin --release``
in project root

# Known issues
- the knobs do not redraw correctly on small parameter changes
- when both effects are maxed out the sound is fully muted

# TODO
- rethink the names of parameters as they might be confusing
- test the plugin on all platforms
- benchmark the following DCT optimisations:
- > Fixed point numbers
- > 10.1016/j.dsp.2008.11.004

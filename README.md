# node-libobs
POC: Node.js bindings for OBS (libobs) for streaming and recording with node.js.

## Build instructions
In order to build this project, you need to have the following installed:
* npm
* node.js
* rustc
* cargo

Set the environment variable `LIBOBS_INCLUDE_DIR` to the obs include directory is located.
This directory should contain the `obs.h` file.
Set the environment variable `LIBOBS_LIB_DIR` to the location of the obs library directory.
This directory should include `obs.(lib|so|dylib)`.

Then, run the following commands:
```bash
npm install
npm run build
```

You may need to change your working directory to the obs binary directory,
otherwise the program will not work:
```ts
process.chdir('/path/to/obs/bin/dir');
```
or
```ts
import { Obs } from 'libobs';

// You'll need to add your obs binary directory
// to your PATH for this to work
process.chdir(await Obs.findObs(true));
```

## Example
```ts
import {
    Obs,
    ObsSettings,
    SpeakerLayout,
    ScaleType,
    VideoColorSpace,
    VideoRange,
    GraphicsModule,
    VideoFormat
} from 'libobs';

// Note: Only one instance of OBS can be created at a time.
const obs = await Obs.newInstance('en-US');

// Get all modules which may be loaded
const modules = await obs.getAllModules('/path/to/your/obs/installation');
// Load the modules
await obs.loadModules(modules);

// Reset the audio and video
await obs.resetAudio({
   fixedBuffering: false,
   speakers: SpeakerLayout.Stereo,
   maxBufferingMs: 1000,
   samplesPerSec: 48000,
});

await obs.resetVideo({
   adapter: 0,
   baseHeight: 1440,
   baseWidth: 2560,
   outputHeight: 1440,
   outputWidth: 2560,
   scaleType: ScaleType.Bicubic,
   colorspace: VideoColorSpace.CS709,
   fpsDen: 1,
   fpsNum: 60,
   gpuConversion: true,
   range: VideoRange.Partial,
   graphicsModule: GraphicsModule.D3D11,
   outputFormat: VideoFormat.NV12,
});

// Create a video encoder
const videoEncoder = await obs.createVideoEncoder('nvenc', 'jim_nvenc', new ObsSettings({
    rate_control: 'CQP',
    cqp: 23,
    preset: 'medium',
    profile: 'high',
}));

// Create an audio encoder
const audioEncoder = await obs.createAudioEncoder('aac', 'jim_aac', new ObsSettings({
   bitrate: 128,
   rate_control: 'CBR',
}));

// Create a new video source
const videoSource = await obs.createSource('screen_capture', 'monitor_capture', 0, new ObsSettings({
   capture_cursor: true,
   monitor: 0,
   method: 2,
}));

// Create a new audio source
await obs.createSource('audio_capture', 'wasapi_output_capture', 1);

// Create a new flv output
const out = await obs.createOutput('flv_output', 'output', new ObsSettings({
   path: '/path/to/your/output.flv',
}));

// Start the output
out.start(videoEncoder, audioEncoder);

// Wait for 10 seconds
await new Promise(resolve => setTimeout(resolve, 10000));

// Stop the output
out.stop();
```
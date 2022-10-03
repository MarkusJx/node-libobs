const obs = require('./index');

const skip = [
    'frontend-tools',
    'obs-websocket',
    //'win-wasapi',
    'win-mf'
];

const modulesToSkip = [
    'aja-output-ui',
    'aja',
    'chrome_elf',
    'decklink',
    'enc-amf',
    'libcef',
    'libEGL',
    'libGLESv2',
    'win-decklink',
    //'win-dshow',
    //'win-capture',
    ...skip
];

async function main() {
    process.chdir(await obs.Obs.findObs(true));
    const instance = await obs.Obs.newInstance('en-US');
    const modules = await instance.getAllModules();
    instance.loadModulesSync(modules.filter(m => !modulesToSkip.includes(m.name)), false);
    console.log('Failed modules:', instance.failedModules);
    console.log('Loaded modules:', instance.loadedModules);

    await instance.resetAudio({
        fixedBuffering: false,
        speakers: obs.SpeakerLayout.Stereo,
        maxBufferingMs: 1000,
        samplesPerSec: 48000,
    });

    await instance.resetVideo({
        adapter: 0,
        baseHeight: 1440,
        baseWidth: 2560,
        outputHeight: 1440,
        outputWidth: 2560,
        scaleType: obs.ScaleType.Bicubic,
        colorspace: obs.VideoColorSpace.CS709,
        fpsDen: 1,
        fpsNum: 60,
        gpuConversion: true,
        range: obs.VideoRange.Partial,
        graphicsModule: obs.GraphicsModule.D3D11,
        outputFormat: obs.VideoFormat.NV12,
    });

    const videoEncoder = await instance.createVideoEncoder('nvenc', 'jim_nvenc', new obs.ObsSettings({
        rate_control: 'CQP',
        cqp: 23,
        preset: 'medium',
        profile: 'high',
    }));

    const audioEncoder = await instance.createAudioEncoder('aac', 'ffmpeg_aac', new obs.ObsSettings()
        .setString('rate_control', 'CBR')
        .setInt('bitrate', 192));

    await instance.createSource('screen_capture', 'monitor_capture', 0, new obs.ObsSettings()
        .setBool('capture_cursor', false)
        .setInt('monitor', 1)
        .setInt('method', 2));
    await instance.createSource('audio_capture', 'wasapi_output_capture', 1);
    //console.log(audio.getProperties().getProperties())

    const out = await instance.createOutput('flv_output', 'output', new obs.ObsSettings()
        .setString('path', "C:/Users/marku/Desktop/test.flv"));
    console.log(await instance.listOutputTypes());

    out.start(videoEncoder, audioEncoder);

    await new Promise(resolve => setTimeout(resolve, 10000));
    out.stop();
}

main().then();
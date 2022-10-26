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
    'rtmp-services',
    //'win-dshow',
    //'win-capture',
    ...skip
];

async function main() {
    process.chdir(await obs.Obs.findObs(true));
    const instance = await obs.Obs.newInstance('en-US', {
        shutdown: true
    });
    //return instance.main();

    const modules = await instance.getAllModules();
    //return console.log(modules);
    await instance.loadModules(modules.filter(m => !skip.includes(m.name)), false);
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

    console.log(await instance.listEncoderTypes())
    console.log(1);
    const videoEncoder = await instance.createVideoEncoder('nvenc', 'jim_nvenc', instance.newSettings({
        rate_control: 'CQP',
        cqp: 23,
        preset: 'medium',
        profile: 'high',
    }));

    console.log(2);
    const audioEncoder = await instance.createAudioEncoder('aac', 'ffmpeg_opus', instance.newSettings({
        rate_control: 'CBR',
        bitrate: 128,
    }));

    console.log(3);
    await instance.createSource('screen_capture', 'monitor_capture', 0, instance.newSettings({
        capture_cursor: false,
        monitor: 1,
        method: 2
    }));
    await instance.createSource('audio_capture', 'wasapi_output_capture', 1);
    //console.log(audio.getProperties().getProperties())

    console.log(4);
    const out = await instance.createOutput('output', 'ffmpeg_muxer', instance.newSettings({
        path: "C:\\Users\\marku\\Desktop\\test.mkv"
    }));
    console.log(await instance.listOutputTypes());

    out.start(videoEncoder, audioEncoder);

    await new Promise(resolve => setTimeout(resolve, 1000));
    try {
        out.pause();
    } catch (e) {
        //out.pause();
        console.error(e)
    }

    await new Promise(resolve => setTimeout(resolve, 1000));
    try {
        out.resume();
    } catch (e) {
        console.error(e)
    }

    await new Promise(resolve => setTimeout(resolve, 1000));
    out.stop();
}

main().then();
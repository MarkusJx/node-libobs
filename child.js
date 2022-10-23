const Commands = {
    PAUSE: 'pause',
    RESUME: 'resume',
    STOP: 'stop',
}

async function main() {
    const obs = require("./index");

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
        'frontend-tools',
        'obs-websocket',
        //'win-wasapi',
        'win-mf'
    ];

    process.chdir(obs.Obs.findObsSync(true));
    const instance = await obs.Obs.newInstance('en-US', {
        shutdown: true
    });
    const modules = await instance.getAllModules();
    //return console.log(modules);
    await instance.loadModules(modules.filter(m => !modulesToSkip.includes(m.name)), false);
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

    const videoEncoder = await instance.createVideoEncoder('nvenc', 'jim_nvenc', instance.newSettings({
        rate_control: 'CQP',
        cqp: 23,
        preset: 'medium',
        profile: 'high',
    }));

    const audioEncoder = await instance.createAudioEncoder('aac', 'ffmpeg_aac', instance.newSettings()
        .setString('rate_control', 'CBR')
        .setInt('bitrate', 192));

    await instance.createSource('screen_capture', 'monitor_capture', 0, instance.newSettings()
        .setBool('capture_cursor', false)
        .setInt('monitor', 1)
        .setInt('method', 2));
    await instance.createSource('audio_capture', 'wasapi_output_capture', 1);
    //console.log(audio.getProperties().getProperties())

    const out = await instance.createOutput('flv_output', 'output', instance.newSettings()
        .setString('path', "C:/Users/marku/Desktop/test.flv"));
    console.log(await instance.listOutputTypes());

    out.start(videoEncoder, audioEncoder);

    process.on('message', message => {
        switch (message) {
            case Commands.STOP:
                out.stop();
                break;
            case Commands.RESUME:
                out.resume();
                break;
            case Commands.PAUSE:
                out.pause();
                break;
        }
    });
}

if (process.argv[2] === 'child') {
    main().then();
} else {
    const {fork} = require('node:child_process');
    const controller = new AbortController();
    const child = fork(__filename, ['child'], {
        signal: controller.signal
    });
    child.on('error', (err) => {
        console.error(err);
        // This will be called with err being an AbortError if the controller aborts
    });

    child.on('exit', (code, signal) => {
        console.log('Child exited with code', code, 'and signal', signal);
    });
    //controller.abort(); // Stops the child process

    setTimeout(() => {
        child.send(Commands.PAUSE);
    }, 5000);

    setTimeout(() => {
        child.send(Commands.RESUME);
    }, 10000);

    setTimeout(() => {
        child.send(Commands.STOP);
    }, 15000);
}
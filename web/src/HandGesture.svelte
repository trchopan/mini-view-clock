<script lang="ts">
    import {
        GestureRecognizer,
        FilesetResolver,
        DrawingUtils,
    } from '@mediapipe/tasks-vision'
    import {onMount} from 'svelte'

    let gestureRecognizer: GestureRecognizer
    const videoHeight = '240px'
    const videoWidth = '360px'
    let video: HTMLVideoElement

    let lastVideoTime = -1
    let results = undefined
    let canvasElement: HTMLCanvasElement
    let canvasCtx: CanvasRenderingContext2D
    let gestureRunning = false
    let creatingRecognizer = false
    let stream: MediaStream

    onMount(() => {
        canvasCtx = canvasElement.getContext('2d')
    })

    const createGestureRecognizer = async () => {
        creatingRecognizer = true
        const vision = await FilesetResolver.forVisionTasks(
            'https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@0.10.3/wasm'
        )
        gestureRecognizer = await GestureRecognizer.createFromOptions(vision, {
            baseOptions: {
                modelAssetPath:
                    'https://storage.googleapis.com/mediapipe-models/gesture_recognizer/gesture_recognizer/float16/1/gesture_recognizer.task',
                delegate: 'GPU',
            },
            runningMode: 'VIDEO',
        })
        creatingRecognizer = false
    }

    const loadGesture = async () => {
        if (!gestureRecognizer) {
            alert('Please wait for gestureRecognizer to load')
            await createGestureRecognizer()
        }

        gestureRunning = !gestureRunning

        turnOnCam()
    }

    const turnOnCam = async () => {
        gestureRunning = true
        const constraints = {
            video: true,
        }

        stream = await navigator.mediaDevices.getUserMedia(constraints)

        video.srcObject = stream
        video.addEventListener('loadeddata', () => {
            console.log('naniii')
            predictWebcam()
        })
    }

    const turnOffCam = () => {
        const tracks = stream.getTracks()
        tracks.forEach(function (track) {
            console.log('👉 track>>', track)
            track.stop()
        })
        gestureRunning = false
    }

    async function predictWebcam() {
        let nowInMs = Date.now()
        if (video.currentTime !== lastVideoTime) {
            lastVideoTime = video.currentTime
            results = gestureRecognizer.recognizeForVideo(video, nowInMs)
        }

        canvasCtx.save()
        canvasCtx.clearRect(0, 0, canvasElement.width, canvasElement.height)
        const drawingUtils = new DrawingUtils(canvasCtx)

        canvasElement.style.height = videoHeight
        video.style.height = videoHeight
        canvasElement.style.width = videoWidth
        video.style.width = videoWidth

        if (results.landmarks) {
            for (const landmarks of results.landmarks) {
                drawingUtils.drawConnectors(
                    landmarks,
                    GestureRecognizer.HAND_CONNECTIONS,
                    {
                        color: '#00FF00',
                        lineWidth: 5,
                    }
                )
                drawingUtils.drawLandmarks(landmarks, {
                    color: '#FF0000',
                    lineWidth: 2,
                })
            }
        }
        canvasCtx.restore()
        if (results.gestures.length > 0) {
            const categoryName = results.gestures[0][0].categoryName
            const categoryScore = parseFloat(
                // @ts-ignore
                results.gestures[0][0].score * 100
            ).toFixed(2)
            const handedness = results.handednesses[0][0].displayName
            // console.log(
            //     `GestureRecognizer: ${categoryName}\n Confidence: ${categoryScore} %\n Handedness: ${handedness}`
            // )
        }
        // Call this function again to keep predicting when the browser is ready.
        if (gestureRunning === true) {
            window.requestAnimationFrame(predictWebcam)
        }
    }
</script>

<div class="hg-container">
    <div class="hg-wrap">
        {#if !gestureRecognizer}
            <button on:click={() => loadGesture()}>Load Gesture</button>
        {:else}
            <button on:click={() => turnOffCam()}>Turn off</button>
            <button on:click={() => turnOnCam()}>Turn on</button>
        {/if}
        <!-- svelte-ignore a11y-media-has-caption -->
        <div style="position: relative;">
            <video bind:this={video} autoplay playsinline></video>
            <canvas
                bind:this={canvasElement}
                class="output_canvas"
                width={videoWidth}
                height={videoHeight}
                style="position: absolute; left: 0px; top: 0px;"
            ></canvas>
        </div>
    </div>
</div>

<style>
    .hg-container {
        position: fixed;
        top: 0;
        left: 0;
        bottom: 0;
        right: 0;
        display: flex;
        justify-content: center;
        align-items: center;
    }
    .hg-wrap {
        display: flex;
        flex-direction: column;
    }
</style>

<script lang="ts">
    import {
        GestureRecognizer,
        FilesetResolver,
        DrawingUtils,
    } from '@mediapipe/tasks-vision'
    import {debounce} from 'lodash'
    import {onMount} from 'svelte'

    const demosSection = document.getElementById('demos')
    let gestureRecognizer: GestureRecognizer
    let runningMode = 'IMAGE'
    let enableWebcamButton: HTMLButtonElement
    let webcamRunning: Boolean = false
    const videoHeight = '360px'
    const videoWidth = '480px'

    // Before we can use HandLandmarker class we must wait for it to finish
    // loading. Machine Learning models can be large and take a moment to
    // get everything needed to run.
    const createGestureRecognizer = async () => {
        const vision = await FilesetResolver.forVisionTasks(
            'https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@0.10.3/wasm'
        )
        gestureRecognizer = await GestureRecognizer.createFromOptions(vision, {
            baseOptions: {
                modelAssetPath:
                    'https://storage.googleapis.com/mediapipe-models/gesture_recognizer/gesture_recognizer/float16/1/gesture_recognizer.task',
                delegate: 'GPU',
            },
            // @ts-ignore
            runningMode: runningMode,
        })
        demosSection.classList.remove('invisible')
    }
    createGestureRecognizer()

    onMount(() => {
        const video = document.getElementById('webcam')
        const canvasElement = document.getElementById(
            'output_canvas'
        ) as HTMLCanvasElement
        const canvasCtx = canvasElement.getContext('2d')
        const gestureOutput = document.getElementById('gesture_output')

        // Check if webcam access is supported.
        function hasGetUserMedia() {
            return !!(
                navigator.mediaDevices && navigator.mediaDevices.getUserMedia
            )
        }

        // If webcam supported, add event listener to button for when user
        // wants to activate it.
        if (hasGetUserMedia()) {
            enableWebcamButton = document.getElementById('webcamButton')
            enableWebcamButton.addEventListener('click', enableCam)
        } else {
            console.warn('getUserMedia() is not supported by your browser')
        }

        // Enable the live webcam view and start detection.
        function enableCam(event) {
            if (!gestureRecognizer) {
                alert('Please wait for gestureRecognizer to load')
                return
            }

            if (webcamRunning === true) {
                webcamRunning = false
                enableWebcamButton.innerText = 'ENABLE PREDICTIONS'
            } else {
                webcamRunning = true
                enableWebcamButton.innerText = 'DISABLE PREDICTIONS'
            }

            // getUsermedia parameters.
            const constraints = {
                video: true,
            }

            // Activate the webcam stream.
            navigator.mediaDevices
                .getUserMedia(constraints)
                .then(function (stream) {
                    video.srcObject = stream
                    video.addEventListener('loadeddata', predictWebcam)
                })
        }

        let lastVideoTime = -1
        let results = undefined
        async function predictWebcam() {
            const webcamElement = document.getElementById('webcam')
            // Now let's start detecting the stream.
            if (runningMode === 'IMAGE') {
                runningMode = 'VIDEO'
                await gestureRecognizer.setOptions({runningMode: 'VIDEO'})
            }
            let nowInMs = Date.now()
            if (video.currentTime !== lastVideoTime) {
                lastVideoTime = video.currentTime
                results = gestureRecognizer.recognizeForVideo(video, nowInMs)
            }

            canvasCtx.save()
            canvasCtx.clearRect(0, 0, canvasElement.width, canvasElement.height)
            const drawingUtils = new DrawingUtils(canvasCtx)

            canvasElement.style.height = videoHeight
            webcamElement.style.height = videoHeight
            canvasElement.style.width = videoWidth
            webcamElement.style.width = videoWidth

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
                gestureOutput.style.display = 'block'
                gestureOutput.style.width = videoWidth
                const categoryName = results.gestures[0][0].categoryName
                const categoryScore = parseFloat(
                    // @ts-ignore
                    results.gestures[0][0].score * 100
                ).toFixed(2)
                const handedness = results.handednesses[0][0].displayName
                gestureOutput.innerText = `GestureRecognizer: ${categoryName}\n Confidence: ${categoryScore} %\n Handedness: ${handedness}`

                if (+categoryScore < 0.7) {
                    console.log(">>> here")
                    return
                }

                if (categoryName === 'Victory') {
                    runCommand('yabai -m window --grid 1:2:0:0:1:1')
                }

                if (categoryName === 'Pointing_Up') {
                    runCommand('yabai -m window --grid 1:2:1:0:1:1')
                }

                if (categoryName === 'Open_Palm') {
                    runCommand('yabai -m window --grid 1:1:0:0:1:1')
                }

                if (categoryName === 'ILoveYou') {
                    runCommand('yabai -m window --grid 8:12:2:1:8:6')
                }

                console.log(">>> command", categoryName)
            } else {
                gestureOutput.style.display = 'none'
            }
            // Call this function again to keep predicting when the browser is ready.
            if (webcamRunning === true) {
                window.requestAnimationFrame(predictWebcam)
            }
        }
    })

    const runCommand = debounce((command: string) => {
        fetch('/run-command', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json', // Set the content type to JSON
            },
            body: JSON.stringify({
                command,
            }),
        })
    }, 200)
</script>

<section id="demos" class="invisible">
    <div id="liveView" class="videoView">
        <button id="webcamButton" class="mdc-button mdc-button--raised">
            <span class="mdc-button__ripple"></span>
            <span class="mdc-button__label">ENABLE WEBCAM</span>
        </button>
        <div style="position: relative;">
            <!-- svelte-ignore a11y-media-has-caption -->
            <video id="webcam" autoplay playsinline></video>
            <canvas
                class="output_canvas"
                id="output_canvas"
                width="1280"
                height="720"
                style="position: absolute; left: 0px; top: 0px;"
            ></canvas>
            <p id="gesture_output" class="output"></p>
        </div>
    </div>
</section>

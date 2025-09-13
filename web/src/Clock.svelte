<script lang="ts">
    import {onDestroy, onMount} from 'svelte'
    import dayjs from 'dayjs'

    let dateStr = ''
    let timeStr = ''
    let secondStr = ''
    let interval: number | null = null

    onMount(() => {
        interval = setInterval(() => {
            const now = new Date()
            dateStr = dayjs(now).format('ddd, DD-MM-YYYY')
            timeStr = dayjs(now).format('HH:mm')
            secondStr = dayjs(now).format('ss')
        }, 1000)
    })

    onDestroy(() => {
        if (interval) {
            clearInterval(interval)
        }
    })
</script>

<svg
    xmlns="http://www.w3.org/2000/svg"
    xmlns:xlink="http://www.w3.org/1999/xlink"
    version="1.1"
    width="640"
    height="300"
    viewBox="0 0 640 300"
    xml:space="preserve"
>
    <g transform="matrix(2.74 0 0 2.74 320.79 49.21)" style="">
        <text
            xml:space="preserve"
            font-family="'Open Sans', sans-serif"
            font-size="18"
            font-style="normal"
            font-weight="normal"
            style="stroke: none; stroke-width: 1; stroke-dasharray: none; stroke-linecap: butt; stroke-dashoffset: 0; stroke-linejoin: miter; stroke-miterlimit: 4; opacity: 1; white-space: pre;"
            class="date"
        >
            <tspan x="-67.16" y="5.65">{dateStr}</tspan>
        </text>
    </g>
    <g transform="matrix(3.71 0 0 3.71 315.76 190)" style="">
        <text
            xml:space="preserve"
            font-family="'Open Sans', sans-serif"
            font-size="60"
            font-style="normal"
            font-weight="normal"
            style="stroke: none; stroke-width: 1; stroke-dasharray: none; stroke-linecap: butt; stroke-dashoffset: 0; stroke-linejoin: miter; stroke-miterlimit: 4; opacity: 1; white-space: pre;"
            style:fill="white"
            class="time"
        >
            <tspan x="-76.49" y="18.85">{timeStr}</tspan>
        </text>
    </g>
    <g transform="matrix(1.3 0 0 1.3 615 120)" style="">
        <text
            xml:space="preserve"
            font-family="'Open Sans', sans-serif"
            font-size="18"
            font-style="normal"
            font-weight="normal"
            style="stroke: none; stroke-width: 1; stroke-dasharray: none; stroke-linecap: butt; stroke-dashoffset: 0; stroke-linejoin: miter; stroke-miterlimit: 4; opacity: 1; white-space: pre;"
            style:fill="white"
            class="time-second"
        >
            <tspan x="-10.29" y="5.65">{secondStr}</tspan>
        </text>
    </g>
</svg>

<style>
    .date {
        fill: #686868;
    }
    text {
        font-family: 'Play', sans-serif;
    }
    svg {
        width: 100%;
        height: 100%;
    }
</style>

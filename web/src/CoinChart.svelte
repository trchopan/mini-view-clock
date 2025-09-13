<script lang="ts">
    import {onMount, onDestroy} from 'svelte'
    import {
        Chart,
        LineController,
        CategoryScale,
        LinearScale,
        PointElement,
        LineElement,
        Tooltip,
        Legend,
    } from 'chart.js'
    import dayjs from 'dayjs'

    // Register Chart.js components globally
    Chart.register(
        LineController,
        CategoryScale,
        LinearScale,
        PointElement,
        LineElement,
        Tooltip,
        Legend
    )

    export let prices: [number, number][] = []
    export let coinName: string

    let canvasElement: HTMLCanvasElement
    let chart: Chart | null = null

    const createChart = () => {
        if (!canvasElement || !prices || prices.length === 0) {
            return
        }

        if (chart) {
            chart.destroy()
        }

        // Prepare data for Chart.js
        const labels = prices.map(p => dayjs(p[0]).format('YYYY MMM DD')) // Format timestamp for x-axis labels
        const data = prices.map(p => p[1]) // Prices for y-axis

        chart = new Chart(canvasElement, {
            type: 'line',
            data: {
                labels: labels,
                datasets: [
                    {
                        label: 'Price (USD)',
                        data: data,
                        borderColor: 'rgb(75, 192, 192)',
                        backgroundColor: 'rgba(75, 192, 192, 0.2)', // Fill color under the line
                        tension: 0.1, // Smooth the line
                        fill: false, // Don't fill area under the line
                        pointRadius: 0, // No points for a cleaner line chart
                        borderWidth: 1.5, // Thinner line
                    },
                ],
            },
            options: {
                responsive: true,
                maintainAspectRatio: false, // Allow canvas to resize freely
                scales: {
                    x: {
                        type: 'category', // Use 'category' for formatted labels
                        title: {
                            display: false,
                        },
                        ticks: {
                            display: false,
                            color: '#ccc', // Tick label color for dark theme
                        },
                    },
                    y: {
                        title: {
                            display: true,
                            text: coinName,
                            color: '#ccc',
                        },
                        ticks: {
                            color: '#ccc',
                        },
                        grid: {
                            color: 'rgba(255, 255, 255, 0.05)',
                        },
                    },
                },
                plugins: {
                    legend: {
                        display: false, // Don't need legend for a single dataset
                        labels: {
                            color: '#ccc',
                        },
                    },
                    tooltip: {
                        mode: 'index',
                        intersect: false, // Show tooltip even if not directly over a point
                        titleColor: '#fff',
                        bodyColor: '#ccc',
                        backgroundColor: 'rgba(0, 0, 0, 0.7)',
                        borderColor: '#303030',
                        borderWidth: 1,
                    },
                },
            },
        })
    }

    onMount(() => {
        createChart()
    })

    // Reactive statement to re-create/update chart if prices prop changes
    $: if (prices) {
        createChart()
    }

    onDestroy(() => {
        if (chart) {
            chart.destroy()
        }
    })
</script>

<div class="w-full h-full">
    <canvas bind:this={canvasElement}></canvas>
</div>

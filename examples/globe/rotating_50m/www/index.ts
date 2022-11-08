import { Renderer } from 'rust_d3_geo_example_globe_rotating_50m';

const perf = document.getElementById('perf');

if (perf != null) {

    perf.innerHTML = 'Render Time: ...Calculating';


    // Holds elapsed samples (use to compute the standard deviation).
    const elapsedArray = [];
    // index into the elapsedArray 0..199
    let index = 0;

    console.log('wasm is imported');
    Renderer.new('./world-atlas/world/50m.json', 0)
        .then((renderer) => {
            const canvas = document.getElementById('c')
            if (canvas === null) {
                return
            }
            const context = canvas.getContext('2d');
            if (context == null) {
                return;
            }

            const renderLoop = () => {
                context.clearRect(0, 0, 960, 600);
                const t0 = performance.now();
                renderer.render();
                const t1 = performance.now();

                // Compute the mean elapsed time and compute the standard deviation based on the
                // the last 200 samples.
                const elapsed = (t1 - t0);
                index = (index + 1) % 200;
                elapsedArray[index] = elapsed;
                if (index === 199) {
                    const n = elapsedArray.length;
                    const mean = elapsedArray.reduce((a, b) => a + b, 0) / n;
                    const stdDev = Math.sqrt(elapsedArray.map(x => Math.pow(x - mean, 2)).reduce((a, b) => a + b) / n)
                    const meanString = mean.toPrecision(4);
                    const stdDevString = stdDev.toPrecision(4);
                    perf.innerHTML = `Mean Render Time: ${meanString} +/- ${stdDevString} ms`;
                }

                requestAnimationFrame(renderLoop);
            }

            renderLoop();
        });
}

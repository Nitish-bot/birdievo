import * as sim from 'simulation-wasm';
import { high_res_ctx } from './utils.js';

const simulation = new sim.Simulation();
const viewport = document.getElementById('viewport');

CanvasRenderingContext2D.prototype.drawTriangle = 
    function(x, y, size) {
        this.beginPath();
        this.moveTo(x, y);
        this.lineTo(x + size, y + size);
        this.lineTo(x - size, y + size);
        this.lineTo(x, y);
        this.fillStyle = 'rgb(0, 0, 0)';
        this.fill();
    }

// const viewportWidth = viewport.width;
// const viewportHeight = viewport.height;
const ctx = high_res_ctx(viewport);

for (const animal of simulation.world().animals) {
    console.log(animal.x)
    ctx.drawTriangle(
        animal.x * viewport.width,
        animal.y * viewport.height,
        0.5 * viewport.width,
    )
}

// ctx.drawTriangle(250, 100, 200, 0);

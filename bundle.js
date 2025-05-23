import * as sim from './pkg/simulation_wasm.js';
import { draw_triangle, draw_circle,  high_res_ctx } from './utils.js';

const simulation = new sim.Simulation();
const viewport = document.getElementById('viewport');

CanvasRenderingContext2D.prototype.drawTriangle = draw_triangle;
CanvasRenderingContext2D.prototype.drawCircle = draw_circle;

document.getElementById('train').onclick = () => {
    console.log(simulation.train());
}

const viewportWidth = viewport.width;
const viewportHeight = viewport.height;
const ctx = high_res_ctx(viewport);

function redraw() {
    ctx.clearRect(0, 0, viewportWidth, viewportHeight)

    simulation.step();

    const world = simulation.world();

    for (const food of world.foods) {
        ctx.drawCircle(
            food.x * viewportWidth,
            food.y * viewportHeight,
            0.005 * viewportWidth,
        );
    }

    for (const animal of world.animals) {
        ctx.drawTriangle(
            animal.x * viewportWidth,
            animal.y * viewportHeight,
            0.02 * viewport.width,
            // the 2d renderer context is rotated  by 
            // PI / 2 radians compared to the standard plane
            // used by rust's insides
            animal.rotation - Math.PI / 2,
        )
    }

    requestAnimationFrame(redraw);
}

redraw();
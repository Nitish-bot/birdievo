export function high_res_ctx(viewport) {
    const viewportWidth = viewport.width;
    const viewportHeight = viewport.height;

    const viewportScale = window.devicePixelRatio || 1;

    viewport.width = viewportWidth * viewportScale;
    viewport.height = viewportHeight * viewportScale;

    viewport.style.width = viewportWidth + 'px';
    viewport.style.height = viewportHeight + 'px';

    const ctx = viewport.getContext('2d');

    ctx.scale(viewportScale, viewportScale);
    return ctx;
}

export function draw_triangle(x, y, size, rotation) {
    this.beginPath();
    
    this.moveTo(
        x - Math.sin(rotation) * size * 1.5,
        y + Math.cos(rotation) * size * 1.5
    );
    
    this.lineTo(
        x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
        y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size
    );

    this.lineTo(
        x - Math.sin(rotation - 2.0 / 3.0 * Math.PI) * size,
        y + Math.cos(rotation - 2.0 / 3.0 * Math.PI) * size
    );

    this.lineTo(
        x - Math.sin(rotation) * size * 1.5,
        y + Math.cos(rotation) * size * 1.5
    );

    this.fillStyle = 'rgb(255, 255, 255)';
    this.fill();
    // this.stroke();
}

export function draw_circle(x, y, radius) {
    this.beginPath();

    // We are outlining the center of the arc and it's angle
    // start and end. We go 0 to 2PI for a circle
    this.arc(x, y, radius, 0, 2.0 * Math.PI);

    this.fillStyle = 'rgb(0, 255, 128)';
    this.fill();
}
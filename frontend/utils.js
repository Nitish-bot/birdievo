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

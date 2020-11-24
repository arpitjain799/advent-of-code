import {ReaderWithBuffer} from './ringbuffer.js';

const COMMAND_BEGIN_PATH = 1;
const COMMAND_CLEAR = 2;
const COMMAND_CLOSE_PATH = 3;
const COMMAND_FILL_RECT = 4;
const COMMAND_FILL_SQUARE = 5;
const COMMAND_END_FRAME = 6;
const COMMAND_FILL_STYLE_RGB = 7;
const COMMAND_LINE_WIDTH = 8;
const COMMAND_STROKE_SQUARE = 9;
const COMMAND_STROKE_STYLE_RGB = 10;
const COMMAND_FILL_TEXT = 11;
const COMMAND_SHADOW_BLUR = 12;
const COMMAND_SHADOW_COLOR = 13;
const COMMAND_DONE = 14;
const COMMAND_DELAY = 15;
const COMMAND_SWITCH_LAYER = 16;
const COMMAND_FILL_STYLE_RGBA = 17;
const COMMAND_SET_ASPECT_RATIO = 18;

export default function Renderer(message, layers) {
    const { buffer, offset, length } = message.data;
    const reader = new ReaderWithBuffer(buffer, offset, length);

    let ctx = layers[0];

    // Non-transparent background to look better when saving:
    ctx.fillStyle = 'black';
    //ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    //ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);

    for (let layer of layers) {
        const scale = layer.canvas.width;
        layer.setTransform(scale, 0, 0, scale, 0, 0);
    }

    this.done = false;

    this.render = () => {
        let end_of_frame = false;
        outer:
        while (reader.hasNext()) {
            const command = reader.next();
            switch (command) {
                case COMMAND_CLEAR: {
                    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
                    break;
                }
                case COMMAND_END_FRAME: {
                    end_of_frame = true;
                    break outer;
                }
                case COMMAND_FILL_SQUARE: {
                    let [x, y, size] = [reader.nextFloat(), reader.nextFloat(), reader.nextFloat()];
                    ctx.fillRect(x, y, size, size);
                    break;
                }
                case COMMAND_FILL_RECT: {
                    let [x, y, width, height] = [reader.nextFloat(), reader.nextFloat(), reader.nextFloat(), reader.nextFloat()];
                    ctx.fillRect(x, y, width, height);
                    break;
                }
                case COMMAND_FILL_STYLE_RGB: {
                    let [r, g, b] = [reader.next(), reader.next(), reader.next()];
                    ctx.fillStyle = 'rgb(' + r + ', ' + g + ',' + b + ')';
                    break;
                }
                case COMMAND_FILL_STYLE_RGBA: {
                    let [r, g, b, a] = [reader.next(), reader.next(), reader.next(), reader.nextFloat()];
                    ctx.fillStyle = 'rgba(' + r + ', ' + g + ',' + b + ', ' + a + ')';
                    break;
                }
                case COMMAND_LINE_WIDTH: {
                    ctx.lineWidth = reader.next();
                    break;
                }
                case COMMAND_STROKE_SQUARE: {
                    let [x, y, size] = [reader.next(), reader.next(), reader.next()];
                    ctx.strokeRect(x, y, size, size);
                    break;
                }
                case COMMAND_STROKE_STYLE_RGB: {
                    let [r, g, b] = [reader.next(), reader.next(), reader.next()];
                    ctx.strokeStyle = 'rgb(' + r + ', ' + g + ',' + b + ')';
                    break;
                }
                case COMMAND_FILL_TEXT: {
                    const text = reader.nextString();
                    console.log('Got text', text);
                    const x = reader.nextFloat();
                    const y = reader.nextFloat();
                    ctx.fillText(text, x, y);
                    break;
                }
                case COMMAND_SHADOW_BLUR: {
                    ctx.shadowBlur = reader.next();
                    break;
                }
                case COMMAND_SHADOW_COLOR: {
                    let [r, g, b] = [reader.next(), reader.next(), reader.next()];
                    ctx.shadowColor = 'rgb(' + r + ', ' + g + ',' + b + ')';
                    break;
                }
                case COMMAND_DONE: {
                    console.log('done rendering');
                    this.done = true;
                    break;
                }
                case COMMAND_DELAY: {
                    this.delay = reader.next();
                    return;
                }
                case COMMAND_SWITCH_LAYER: {
                    const activeLayer = reader.next();
                    ctx = layers[activeLayer];
                    return;
                }
                case COMMAND_SET_ASPECT_RATIO: {
                    const newAspectRatio = reader.nextFloat();
                    if (!window.aspectRatio) {
                        window.reloadWithParameters({aspectRatio: newAspectRatio});
                        this.done = true;
                        return;
                    }
                    break;
                }
                default:
                    throw new Error('Unhandled command: ' + command + ', done=' + this.done);
            }
        }

        reader.wantMore();
    }
}

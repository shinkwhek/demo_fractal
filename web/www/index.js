'use strict';
import * as wasm from "hello-wasm-pack";
wasm.greet();

let count = 1;

const draw = (ctx, w, h, pathes) => {
    ctx.clearRect(0, 0, w, h);
    ctx.beginPath();
    ctx.moveTo(0, h / 2);

    for (const path of pathes) {
        ctx.lineTo(path[0], path[1])
    }

    ctx.lineTo(w, h / 2);
    ctx.stroke();
}

const koch = import('../pkg')
    .catch(console.error);


Promise.all([koch]).then(async ([
    { generate_koch_curve_pathes, generate_dragon_pathes }
]) => {

    console.log("finished loading wasm");


    const action = (gen, id) => {
        const CANVAS_ID = id;
        let canvas = document.getElementById(CANVAS_ID);
        let context = canvas.getContext("2d");
        const canvasWidth = canvas.width;
        const canvasHeight = canvas.height;
        const MAX_ITER = count;
        let result = gen(canvasWidth, canvasHeight, MAX_ITER);
        draw(context, canvasWidth, canvasHeight, result);
    };


    const plus = document.getElementById('plus');
    plus.addEventListener('click', () => {
        count++;
        console.log("(add) current:" + count);
    });

    const minus = document.getElementById('minus');
    minus.addEventListener('click', () => {
        if (count != 1) {
            count--;
            console.log('(sub) current:' + count);
        }
    });


    const renderBtn = document.getElementById('render');
    renderBtn.addEventListener('click', () => {
        action(generate_koch_curve_pathes, "koch");
        action(generate_dragon_pathes, "dragon");
    });

});
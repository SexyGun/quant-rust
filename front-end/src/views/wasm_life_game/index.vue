<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { Universe, Cell } from "wasm_life_game";
import { memory } from "wasm_life_game/wasm_life_game_bg.wasm";

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Create a new universe and get its width and height
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Get the canvas element and its context
const canvasRef = ref(null);
// Create a reference to the animation frame
const animationId = ref(null);
// Create a reference to the button text
const buttonText = ref("▶");
// Create a reference to the fps element text
const fpsElementText = ref("");
// Calculate the frames per second
const fps = {
  frames: [],
  lastFrameTimeStamp: performance.now(),
  render() {
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = (1 / delta) * 1000;

    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    fpsElementText.value = `
Frames per Second:
latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
        `.trim();
  },
};

// Render loop
const renderLoop = () => {
  fps.render();
  universe.tick();
  drawGrid();
  drawCells();
  animationId.value = requestAnimationFrame(renderLoop);
};
// Check if the animation is paused
const isPaused = () => {
  return animationId.value === null;
};
// Handle button click event
const clickBtn = () => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
};
// Play the animation
const play = () => {
  buttonText.value = "⏸";
  renderLoop();
};
// Pause the animation
const pause = () => {
  buttonText.value = "▶";
  cancelAnimationFrame(animationId.value);
  animationId.value = null;
};
// Draw grid
const drawGrid = () => {
  const canvas = canvasRef.value;
  const ctx = canvas.getContext("2d");
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * width + column;
};

// Draw cells
const drawCells = () => {
  const canvas = canvasRef.value;
  const ctx = canvas.getContext("2d");
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }
  ctx.stroke();
};

// Handle canvas click event
const clickCanvas = (event) => {
  const canvas = canvasRef.value;
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  universe.toggle_cell(row, col);

  drawGrid();
  drawCells();
};
onMounted(() => {
  const canvas = canvasRef.value;
  canvas.height = (CELL_SIZE + 1) * height + 1;
  canvas.width = (CELL_SIZE + 1) * width + 1;
  drawGrid();
  drawCells();
  play();
});

onUnmounted(() => {
  pause();
});
</script>

<template>
  <div class="container">
    <button class="play-pause" @click="clickBtn">
      {{ buttonText }}
    </button>
    <div class="fps">
      {{ fpsElementText }}
    </div>
    <canvas ref="canvasRef" @click="clickCanvas"></canvas>
  </div>
</template>

<style scoped>
.container {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}
.play-pause {
  width: 75px;
  height: 25px;
  margin-bottom: 10px;
}
.fps {
  white-space: pre;
  font-family: monospace;
  margin-bottom: 10px;
}
</style>
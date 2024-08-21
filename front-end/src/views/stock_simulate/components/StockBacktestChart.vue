<script setup>
import { ref, onMounted, watchEffect } from "vue";
import { Chart, registerables } from "chart.js";
import annotationPlugin from "chartjs-plugin-annotation";
/**
 * annotation 是 chartjs-plugin-annotation 插件的配置选项，它允许你在图表上添加各种类型的注释。
 * 这些注释可以是线、框或者点，你可以自定义它们的位置、颜色、大小等属性。
 */
Chart.register(annotationPlugin);
Chart.register(...registerables);

const props = defineProps({
  labels: {
    type: Array,
    required: true,
  },
  priceData: {
    type: Array,
    required: true,
  },
  strategyData: {
    type: Array,
    required: true,
  },
  buyAndSellSignal: {
    type: Array,
    required: true,
  },
});

const chart = ref(null);
const chartInstance = ref(null);

const createChart = () => {
  console.log("createChart");
  if (chartInstance.value) {
    chartInstance.value.destroy();
  }
  const ctx = chart.value.getContext("2d");
  chartInstance.value = new Chart(ctx, {
    type: "line",
    data: {
      labels: props.labels,
      datasets: [
        {
          label: "收盘价",
          data: props.priceData,
          borderColor: "#9d2933",
          backgroundColor: "#9d2933",
          yAxisID: "y",
        },
        {
          label: "累计最大收盘价",
          data: props.strategyData,
          borderColor: "#177cb0",
          backgroundColor: "#177cb0",
          yAxisID: "y",
        },
      ],
    },
    options: {
      scales: {
        y: {
          type: "linear",
          position: "left",
          beginAtZero: false,
          title: {
            display: true,
            text: "股价",
          },
        },
      },
      plugins: {
        legend: {
          position: "top",
        },
        title: {
          display: true,
          text: "模拟交易",
        },
        annotation: {
          annotations: props.buyAndSellSignal
            .filter((signal) => signal.type != "none")
            .map((signal) => ({
              type: "line",
              xMin: signal.x,
              xMax: signal.x,
              yMin: signal.type === "buy" ? signal.y + 0.5 : signal.y - 0.5,
              yMax: signal.y,
              borderColor: signal.type === "buy" ? "red" : "green",
              borderWidth: 2,
              label: {
                content: signal.type === "buy" ? "买入" : "卖出",
                display: true,
                position: "start",
                yAdjust: signal.type === "buy" ? -10 : 10,
              },
              arrowHeads: {
                end: {
                  display: true,
                  length: 10,
                },
              },
            })),
        },
      },
    },
  });
};

onMounted(createChart);

watchEffect(() => {
  if (
    props.labels.length > 0 &&
    props.priceData.length > 0 &&
    props.strategyData.length > 0
  ) {
    createChart();
  }
});
</script>

<template>
  <div>
    <canvas ref="chart"></canvas>
  </div>
</template>

<style>
.test {
  color: rgba(35, 174, 243, 0.877);
}
</style>
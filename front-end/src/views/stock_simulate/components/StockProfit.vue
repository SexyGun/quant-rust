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
  standerdData: {
    type: Array,
    required: true,
  },
  strategyData: {
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
          label: "基准收益",
          data: props.standerdData,
          borderColor: "#177cb0",
          backgroundColor: "#177cb0",
          yAxisID: "y",
        },
        {
          label: "趋势突破策略收益",
          data: props.strategyData,

          borderColor: "#9d2933",
          backgroundColor: "#9d2933",
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
        },
      },
      plugins: {
        legend: {
          position: "top",
        },
        title: {
          display: true,
          text: "计算基准收益/趋势突破策略收益",
        },
      },
    },
  });
};

onMounted(createChart);

watchEffect(() => {
  console.log("standerdData", props.standerdData);
  console.log("strategyData", props.strategyData);
  if (props.standerdData.length > 0 && props.strategyData.length > 0) {
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
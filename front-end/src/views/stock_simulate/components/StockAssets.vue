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
  assetsdData: {
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
          label: "资产",
          data: props.assetsdData,
          borderColor: "#9d2933",
          backgroundColor: "#9d2933",
          yAxisID: "y",
        },
      ],
    },
    options: {
      scales: {
        x: {
          ticks: {
            callback: function (value, index, values) {
              let label = props.labels[index];
              // 每隔5个标签显示一个
              return index % 5 === 0 ? label : "";
            },
          },
        },
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
          text: "资产曲线",
        },
      },
    },
  });
};

onMounted(createChart);

watchEffect(() => {
  if (props.assetsdData.length > 0) {
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
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
  const y_step =
    (props.strategyData[props.strategyData.length - 1] -
      props.strategyData[0]) /
    15;
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
          annotations: props.buyAndSellSignal.map((signal) => ({
            type: "line",
            xMin: signal.operate_date,
            xMax: signal.operate_date,
            yMin:
              signal.order_type === "Buy"
                ? signal.close + y_step
                : signal.close - y_step,
            yMax: signal.close,
            borderColor: signal.order_type === "Buy" ? "red" : "green",
            borderWidth: 2,
            label: {
              content:
                signal.order_type === "Buy"
                  ? `买入 ${signal.operate_num} 股`
                  : `卖出 ${signal.operate_num} 股`,
              display: true,
              position: "start",
              yAdjust: signal.order_type === "Buy" ? -10 : 10,
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
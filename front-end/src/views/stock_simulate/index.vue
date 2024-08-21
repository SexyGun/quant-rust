<script setup>
import { ref, computed } from "vue";
import { ElButton } from "element-plus";
import { test_api } from "./api";
import StockBacktestChart from "./components/StockBacktestChart.vue";
import StockProfit from "./components/StockProfit.vue";
// data
const stockData = ref({
  df_stock: [],
  best_param: [],
  buy_days_query: [],
  sell_days_query: [],
});

// computed
const labels = computed(() =>
  stockData.value.df_stock.map((item) => item.date)
);
const closeData = computed(() =>
  stockData.value.df_stock.map((item) => item.close)
);
const maxCloseData = computed(() => {
  let acc_max = 0;
  let result = stockData.value.df_stock.map((item) => {
    acc_max = Math.max(acc_max, item.close);
    return acc_max;
  });
  return result;
});
const buyAndSellSignal = computed(() =>
  stockData.value.df_stock.map((item) => {
    if (stockData.value.buy_days_query.includes(item.date)) {
      return {
        x: item.date,
        y: item.close,
        type: "buy",
      };
    } else if (stockData.value.sell_days_query.includes(item.date)) {
      return {
        x: item.date,
        y: item.close,
        type: "sell",
      };
    } else {
      return {
        x: item.date,
        y: item.close,
        type: "none",
      };
    }
  })
);
const standerdData = computed(() => {
  let base_close = stockData.value.df_stock[0]?.close || 0;
  return stockData.value.df_stock.map((item) => {
    let col_result = Math.log(item.close / base_close);
    base_close = item.close;
    return col_result;
  });
});
const strategyData = computed(() => {
  return standerdData.value.map((item, index) => {
    return item * (stockData.value.df_stock[index].signal || 0);
  });
});

// methods
const test = async () => {
  const res = await test_api({
    code: "600611.SH",
  });
  console.log("test", res);
  stockData.value = res.data || {
    df_stock: [],
    best_param: [],
    buy_days_query: [],
    sell_days_query: [],
  };
};
</script>

<template>
  <div class="container">
    <div
      class="content"
      style="justify-content: flex-start; width: 100%; margin: 0px auto"
    >
      <h1>Reports</h1>
    </div>
    <ElButton @click="test">test</ElButton>
    <div class="content">
      <div class="chart-card">
        <StockBacktestChart
          :labels="labels"
          :priceData="closeData"
          :strategyData="maxCloseData"
          :buyAndSellSignal="buyAndSellSignal"
        />
      </div>
      <div class="chart-card">
        <StockProfit
          :labels="labels"
          :standerdData="standerdData"
          :strategyData="strategyData"
        />
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.container {
  background: #fafafa;
  font-family: "Source Sans Pro", helvetivca, arial;
  font-size: 16px;
  color: #505e67;
  padding: 10px;
  height: 100vh;
}
h1 {
  font-family: "Bree Serif", Georgia, serif;
  font-size: 30px;
  font-weight: normal;
  width: 100%;
  border-bottom: 1px solid #e1e1e1;
  padding-bottom: 20px;
}
.content {
  display: flex;
  justify-content: center;
  flex-flow: row-wrap;
  -webkit-flex-flow: row wrap;
}
.container .chart-card {
  background: #ffffff;
  width: 100%;
  max-width: 1280px;
  box-sizing: border-box;
  padding: 25px 20px 10px 10px;
  margin: 1em;
  border: 1px solid #e4e4e4;
  border-radius: 3px;
  &:hover {
    box-shadow: rgba(0, 0, 0, 0.1) 0px 3px 0px -2px;
  }
}
</style>


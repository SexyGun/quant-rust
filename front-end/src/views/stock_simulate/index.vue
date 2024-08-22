<script setup>
import { ref, computed, onMounted } from "vue";
import {
  ElButton,
  ElInput,
  ElForm,
  ElFormItem,
  ElRow,
  ElCol,
  ElInputNumber,
} from "element-plus";
import { test_api } from "./api";
import StockBacktestChart from "./components/StockBacktestChart.vue";
import StockProfit from "./components/StockProfit.vue";
import StockAssets from "./components/StockAssets.vue";

// mounted
onMounted(() => {
  let params = new URLSearchParams(window.location.search);
  let code = params.get("code"); // 'myParam' 是你想获取的参数的名称
  searchForm.value.code = code || "";
});
// data
const stockData = ref({
  df_stock: [],
  best_param: [],
  operate_record: [],
});
const searchForm = ref({
  code: "",
  assets: 100000,
  n1_min: 5,
  n1_max: 20,
  n2_min: 1,
  n2_max: 15,
  win_min: 1.5,
  win_max: 2.5,
  loss_min: 0.5,
  loss_max: 1.5,
  adjust_min: 0,
  adjust_max: 100,
});
const formRef = ref(null);

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
const buyAndSellSignal = computed(() => stockData.value.operate_record || []);
const standerdData = computed(() => {
  let base_close = stockData.value.df_stock[0]?.close || 0;
  let acc_num = 0;

  return stockData.value.df_stock.map((item) => {
    let col_result = Math.log(item.close / base_close);
    base_close = item.close;
    acc_num = col_result + acc_num;
    return acc_num;
  });
});
const strategyData = computed(() => {
  let acc_num = 0;
  return standerdData.value.map((item, index) => {
    acc_num = item * (stockData.value.df_stock[index].signal || 0) + acc_num;
    return acc_num;
  });
});
const assetsdData = computed(() =>
  stockData.value.df_stock.map((item) => item.total_assets)
);
const bestParam = computed(() => {
  const labels = ["n1", "n2", "win", "loss", "adjust"];
  return stockData.value.best_param.map((value, index) => {
    return {
      label: labels[index],
      value: value,
    };
  });
});

// methods
const fetchSimulate = async () => {
  if (!formRef) {
    return;
  }
  await formRef.value.validate();
  const res = await test_api({
    code: searchForm.value.code,
    assets: searchForm.value.assets,
    n1_range:
      searchForm.value.n1_min && searchForm.value.n1_max
        ? [searchForm.value.n1_min, searchForm.value.n1_max]
        : undefined,
    n2_range:
      searchForm.value.n2_min && searchForm.value.n2_max
        ? [searchForm.value.n2_min, searchForm.value.n2_max]
        : undefined,
    win_range:
      searchForm.value.win_min && searchForm.value.win_max
        ? [searchForm.value.win_min, searchForm.value.win_max]
        : undefined,
    loss_range:
      searchForm.value.loss_min && searchForm.value.loss_max
        ? [searchForm.value.loss_min, searchForm.value.loss_max]
        : undefined,
    adjust_range:
      searchForm.value.adjust_min && searchForm.value.adjust_max
        ? [searchForm.value.adjust_min, searchForm.value.adjust_max]
        : undefined,
  });
  stockData.value = res.data || {
    df_stock: [],
    best_param: [],
    operate_record: [],
  };
};
</script>

<template>
  <div class="container">
    <div
      class="content"
      style="justify-content: space-between; width: 100%; margin: 0px auto"
    >
      <h1>Reports</h1>
      <div style="display: flex">
        <el-form :model="searchForm" label-width="auto" ref="formRef">
          <el-row :gutter="20">
            <el-col :span="8">
              <el-form-item label="TS代码" prop="code" required>
                <el-input
                  v-model="searchForm.code"
                  placeholder="请输入"
                  clearable
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="初始资金" prop="assets" required>
                <ElInputNumber
                  v-model="searchForm.assets"
                  placeholder="请输入"
                  clearable
                  controls-position="right"
                  style="width: 100%"
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="n1_min" prop="n1_min" required>
                <ElInputNumber
                  v-model="searchForm.n1_min"
                  placeholder="请输入"
                  clearable
                  controls-position="right"
                  style="width: 100%"
                  min="0"
                  step="1"
                  step-strictly
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="n1_max" prop="n1_max" required>
                <ElInputNumber
                  v-model="searchForm.n1_max"
                  placeholder="请输入"
                  clearable
                  controls-position="right"
                  style="width: 100%"
                  min="0"
                  step="1"
                  step-strictly
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="n2_min" prop="n2_min" required>
                <ElInputNumber
                  v-model="searchForm.n2_min"
                  placeholder="请输入"
                  clearable
                  controls-position="right"
                  style="width: 100%"
                  min="0"
                  step="1"
                  step-strictly
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="n2_max" prop="n2_max" required>
                <ElInputNumber
                  v-model="searchForm.n2_max"
                  placeholder="请输入"
                  clearable
                  controls-position="right"
                  style="width: 100%"
                  min="0"
                  step="1"
                  step-strictly
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="win_min" prop="win_min" required>
                <ElInputNumber
                  v-model="searchForm.win_min"
                  placeholder="请输入"
                  clearable
                  controls-position="right"
                  style="width: 100%"
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="win_max" prop="win_max" required>
                <ElInputNumber
                  v-model="searchForm.win_max"
                  placeholder="请输入"
                  clearable
                  controls-position="right"
                  style="width: 100%"
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="loss_min" prop="loss_min" required>
                <ElInputNumber
                  v-model="searchForm.loss_min"
                  placeholder="请输入"
                  clearable
                  controls-position="right"
                  style="width: 100%"
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="loss_max" prop="loss_max" required>
                <ElInputNumber
                  v-model="searchForm.loss_max"
                  placeholder="请输入"
                  clearable
                  controls-position="right"
                  style="width: 100%"
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="adjust_min" prop="adjust_min" required>
                <ElInputNumber
                  v-model="searchForm.adjust_min"
                  placeholder="请输入"
                  clearable
                  controls-position="right"
                  style="width: 100%"
                  min="0"
                  step="1"
                  step-strictly
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="adjust_max" prop="adjust_max" required>
                <ElInputNumber
                  v-model="searchForm.adjust_max"
                  placeholder="请输入"
                  clearable
                  controls-position="right"
                  style="width: 100%"
                  min="0"
                  step="1"
                  step-strictly
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-button @click="fetchSimulate(formRef)" type="primary"
                >获取模拟结果</el-button
              >
            </el-col>
          </el-row>
        </el-form>
      </div>
    </div>
    <div class="content">
      <div class="chart-card" style="width: 100%">
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
      <div class="chart-card">
        <StockAssets :labels="labels" :assetsdData="assetsdData" />
      </div>
      <div class="chart-card">
        <ul>
          <li v-for="item in bestParam" :key="item.label">
            {{ item.label }}: {{ item.value }}
          </li>
        </ul>
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
  width: 45%;
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


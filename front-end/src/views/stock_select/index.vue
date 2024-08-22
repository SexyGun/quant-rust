<script setup>
import { ref, computed, onMounted } from "vue";
import { ElDatePicker, ElButton, ElInputNumber } from "element-plus";
import {
  getStockRps,
  getStockRpsList,
  getStockDaily,
  clearStockRps,
} from "./api";
import StockCard from "./components/StockCard.vue";
import moment from "moment";

// mounted
onMounted(() => {
  fetchStockData(moment(searchDate.value).format("YYYYMMDD"));
});

// data
const stockList = ref([
  {
    ts_code: "123",
    name: "比亚迪",
    rps: 99,
    increase: 2,
  },
  {
    ts_code: "1234",
    name: "万科A",
    rps: 99,
    increase: -2,
  },
]);
const searchDate = ref(new Date());
const importStockRpsLoading = ref(false);
const importDailyStockLoading = ref(false);
const clearStockRpsLoading = ref(false);
const searchDateRange = ref(120);

// computed

// methods
const importStockRps = async () => {
  importStockRpsLoading.value = true;
  await getStockRps({
    date: moment(searchDate.value).format("YYYYMMDD"),
    range: searchDateRange.value,
  });
  importStockRpsLoading.value = false;
  changeDate();
};
const importDailyStock = async () => {
  importDailyStockLoading.value = true;
  await getStockDaily();
  importDailyStockLoading.value = false;
};
const disabledDate = (time) => {
  return time.getTime() > Date.now();
};
const changeDate = () => {
  fetchStockData(moment(searchDate.value).format("YYYYMMDD"));
};
const fetchStockData = async (date) => {
  const res = await getStockRpsList({ date });
  stockList.value = res.data || [];
};
// 打开后自己把 name 中的参数复制到搜索栏中搜索
const goToBaidu = (code) => {
  window.open(`/stock-simulate?code=${code}`);
};
const fetchClearStockRps = async () => {
  clearStockRpsLoading.value = true;
  await clearStockRps();
  clearStockRpsLoading.value = false;
};
const colRankChange = (stock) => {
  const { rank_change } = stock;
  if (!rank_change) return "-";

  let styleMap = {
    NoChange: "无变化",
    NewInBoard: "新上榜",
    Increase: "+",
    Decrease: "-",
  };
  if (typeof rank_change === "string") {
    return styleMap[rank_change];
  } else {
    return `${styleMap[Object.keys(rank_change)[0]]} ${
      Object.values(rank_change)[0]
    }`;
  }
};
</script>

<template>
  <div>
    <h1 class="title">Rps 百强股票</h1>
    <div class="tool-bar">
      <div style="display: flex; margin-right: 20px">
        <div style="display: flex; align-items: center">
          <div style="font-weight: 400">无变化：</div>
          <div
            style="
              width: 40px;
              height: 20px;
              background-color: #17141d;
              margin-right: 10px;
            "
          ></div>
        </div>
        <div style="display: flex; align-items: center">
          <div style="font-weight: 400">新上榜：</div>
          <div
            style="
              width: 40px;
              height: 20px;
              background-image: linear-gradient(to bottom, #20002c, #cbb4d4);
              margin-right: 10px;
            "
          ></div>
        </div>
        <div style="display: flex; align-items: center">
          <div style="font-weight: 400">排名上升：</div>
          <div
            style="
              width: 40px;
              height: 20px;
              background-image: linear-gradient(to bottom, #ba8b02, #181818);
              margin-right: 10px;
            "
          ></div>
        </div>
        <div style="display: flex; align-items: center">
          <div style="font-weight: 400">排名下降：</div>
          <div
            style="
              width: 40px;
              height: 20px;
              background-image: linear-gradient(to bottom, #304352, #d7d2cc);
            "
          ></div>
        </div>
      </div>
      <ElDatePicker
        v-model="searchDate"
        placeholder="选择查询日期"
        type="date"
        :disabled-date="disabledDate"
        style="margin-right: 10px"
        @change="changeDate"
      />
      <ElInputNumber
        v-model="searchDateRange"
        placeholder="请输入Rps计算范围"
        clearable
        controls-position="right"
        min="0"
        step="1"
        step-strictly
        style="margin-right: 10px"
      />
      <ElButton
        @click="importStockRps"
        type="primary"
        :loading="importStockRpsLoading"
        >Rps 新数据入库</ElButton
      >
      <ElButton @click="fetchClearStockRps" :loading="clearStockRpsLoading"
        >清空 Rps 数据</ElButton
      >
      <ElButton
        @click="importDailyStock"
        type="primary"
        :loading="importDailyStockLoading"
        >股票新数据入库</ElButton
      >
    </div>
    <div class="card-container">
      <StockCard
        v-for="stock in stockList"
        :key="stock.ts_code"
        @click="goToBaidu(stock.ts_code)"
        :stock="stock"
      >
        <template #title> {{ stock.name }}（{{ stock.ts_code }}） </template>
        <template #content>
          <div class="rps">
            <div class="rps-label">股价强度</div>
            <div class="rps-value">{{ stock.rps }}</div>
          </div>
          <div class="increase">
            <div class="increase-label">区间浮动</div>
            <div
              class="increase-value"
              :class="+stock.increase > 0 ? 'red' : 'green'"
            >
              <span>
                <span>
                  {{ stock.increase }}
                </span>
                <span style="margin-left: 5px">{{
                  +stock.increase > 0 ? "&uarr;" : "&darr;"
                }}</span>
              </span>
            </div>
          </div>
          <div class="rank">
            <div class="rank-label">排名变化</div>
            <div class="rank-value">{{ colRankChange(stock) }}</div>
          </div>
        </template>
      </StockCard>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.title {
  font-family: "Raleway";
  font-size: 24px;
  font-weight: 700;
  color: #5d4037;
  text-align: center;
}

.tool-bar {
  display: flex;
  padding: 20px 40px;
  justify-content: flex-end;
}

.card-container {
  padding: 20px 40px;
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  font-family: sans-serif;
  .rps {
    display: flex;
    .rps-label {
      font-size: 16px;
      width: 75px;
      color: #c0c0c0;
    }
    .rps-value {
      margin-left: 15px;
      color: #ff7272;
    }
  }
  .increase {
    display: flex;
    .increase-label {
      color: #c0c0c0;
      font-size: 16px;
      width: 75px;
    }
    .increase-value {
      margin-left: 15px;
      font-size: 18px;
      &.red {
        color: #f00;
      }
      &.green {
        color: #00e200;
      }
    }
  }
  .rank {
    display: flex;
    .rank-label {
      font-size: 16px;
      width: 75px;
      color: #c0c0c0;
    }
    .rank-value {
      margin-left: 15px;
      color: #05ffea;
    }
  }
}
</style>
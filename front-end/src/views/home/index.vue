<script setup>
import Card from "@/components/Card.vue";
import { testGetStock } from "./api";
const test = () => {
  console.log("test");
  testGetStock();
};
const capabilityList = [
  {
    header: "大盘股票汇总",
    content: "汇集了大盘股票的基本信息。",
    image:
      "https://p2.itc.cn/q_70/images03/20210303/c2b9fb463c8442efa6318ea5fae48fb8.jpeg",
    targetRoute: "/stock",
  },
  {
    header: "股票 RPS 排名",
    content: "Rps（股票价格强度指数）使用最近 N 天，股票的变化幅度进行全局排名，取前 300 进行展示。",
    image: "https://d3mww1g1pfq2pt.cloudfront.net/Avatar/ckm0ait5yrf9z0996wv7idooz/1620092872733.png",
    targetRoute: "/stock-select",
  },
  {
    header: "股票回测",
    content:
      "模拟了单只股票 N 日突破算法的回测结果，并给出了模拟参数。",
    image:
      "https://img1.baidu.com/it/u=1721606346,478465930&fm=253&fmt=auto&app=138&f=JPEG?w=800&h=460",
    targetRoute: "/stock-simulate",
  },
  {
    header: "生命游戏",
    content: "使用 wasm 实现的生命游戏。",
    image:
      "https://images.ctee.com.tw/newsphoto/2021-05-12/1024/20210512700209.jpg",
    targetRoute: "/wasm-life-game",
  },
];
const openTargetRoute = (url) => {
  window.open(url);
}
</script>

<template>
  <div>
    <h1 class="title">选择你的韭菜</h1>
    <!-- <button @click="test">test</button> -->
    <div id="app" class="container">
      <card
        v-for="ca in capabilityList"
        :key="ca.header"
        :data-image="ca.image"
        @click="openTargetRoute(ca.targetRoute)"
      >
        <template #header>
          <h1>{{ ca.header }}</h1>
        </template>
        <template #content>
          <p>{{ ca.content }}</p>
        </template>
      </card>
    </div>
  </div>
</template>

<style lang="scss" scoped>
body {
  margin: 40px 0;
  font-family: "Raleway";
  font-size: 14px;
  font-weight: 500;
  background-color: #bcaaa4;
  -webkit-font-smoothing: antialiased;
}

.title {
  font-family: "Raleway";
  font-size: 24px;
  font-weight: 700;
  color: #5d4037;
  text-align: center;
}

p {
  line-height: 1.5em;
}

h1 + p,
p + p {
  margin-top: 10px;
}

.container {
  padding: 40px 80px;
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
}
</style>
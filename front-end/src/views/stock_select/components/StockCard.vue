<script setup>
import { computed } from "vue";
const props = defineProps({
  stock: {
    type: Object,
    required: true,
  },
});

const cardStyle = computed(() => {
  const { rank_change } = props.stock;
  if (!rank_change) return {};
  let styleMap = {
    NoChange: {},
    NewInBoard: {
      backgroundImage: "linear-gradient(to bottom, #20002c, #cbb4d4)",
    },
    Increase: {
      backgroundImage: "linear-gradient(to bottom, #ba8b02, #181818)",
    },
    Decrease: {
      backgroundImage: "linear-gradient(to bottom, #304352, #d7d2cc)",
    },
  };
  if (typeof rank_change === "string") {
    return styleMap[rank_change];
  } else {
    return styleMap[Object.keys(rank_change)[0]];
  }
});
</script>

<template>
  <div class="card" :style="cardStyle">
    <h3 class="title">
      <slot name="title"></slot>
    </h3>
    <div class="content">
      <slot name="content"></slot>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.card {
  display: flex;
  height: 280px;
  width: 250px;
  background-color: #17141d;
  border-radius: 10px;
  box-shadow: -1rem 0 3rem #000;
  /*   margin-left: -50px; */
  transition: 0.4s ease-out;
  position: relative;
  left: 0px;
}

.card:not(:first-child) {
  margin-left: -75px;
}

.card:hover {
  transform: translateY(-20px);
  transition: 0.4s ease-out;
}

.card:hover ~ .card {
  position: relative;
  left: 50px;
  transition: 0.4s ease-out;
}

.title {
  color: white;
  font-weight: 300;
  position: absolute;
  left: 20px;
  top: 15px;
}

.content {
  position: absolute;
  top: 100px;
  left: 20px;
  width: 200px;
  display: block;
}
</style>

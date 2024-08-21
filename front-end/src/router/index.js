import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '@/views/home/index.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/stock',
      name: 'stockBasic',
      component: () => import('../views/stock_basic/index.vue')
    },
    {
      path: '/stock-select',
      name: 'stockSelect',
      component: () => import('../views/stock_select/index.vue')
    },
    {
      path: '/stock-simulate',
      name: 'stocksimulate',
      component: () => import('../views/stock_simulate/index.vue')
    },
  ]
})

export default router

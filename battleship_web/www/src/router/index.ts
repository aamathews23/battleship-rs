import { createRouter, createWebHistory } from 'vue-router';
import HomeView from '../views/HomeView.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    // lazy-loaded comp when route is visited
    {
      path: '/game',
      name: 'game',
      component: () => import('../views/GameView.vue'),
    },
  ],
});

export default router;

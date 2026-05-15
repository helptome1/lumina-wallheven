import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: () => import('@/components/layout/AppShell.vue'),
      children: [
        { path: '', redirect: '/hot' },
        { path: 'hot', name: 'hot', component: () => import('@/views/HomeView.vue'), meta: { title: '热门推荐', icon: 'Flame' } },
        { path: 'trending', name: 'trending', component: () => import('@/views/HomeView.vue'), meta: { title: '热门趋势', icon: 'TrendingUp' } },
        { path: 'anime', name: 'anime', component: () => import('@/views/AnimeView.vue'), meta: { title: '动漫精选', icon: 'Ghost' } },
        { path: 'people', name: 'people', component: () => import('@/views/PeopleView.vue'), meta: { title: '人物精选', icon: 'Users' } },
        { path: 'collection', name: 'collection', component: () => import('@/views/CollectionView.vue'), meta: { title: '我的收藏', icon: 'Heart' } },
        { path: 'downloads', name: 'downloads', component: () => import('@/views/DownloadView.vue'), meta: { title: '下载中心', icon: 'Download' } },
        { path: 'about', name: 'about', component: () => import('@/views/AboutView.vue'), meta: { title: '关于', icon: 'Info' } },
      ],
    },
  ],
})

export default router

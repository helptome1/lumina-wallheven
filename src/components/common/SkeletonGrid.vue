<template>
  <div class="columns-2 md:columns-3 lg:columns-4 xl:columns-5 gap-4 px-5 pb-8">
    <div
      v-for="i in 20"
      :key="i"
      class="break-inside-avoid mb-4 rounded-2xl overflow-hidden"
      :style="{ height: `${140 + Math.sin(i * 1.7) * 80}px` }"
    >
      <div class="w-full h-full skeleton-pulse rounded-2xl" />
    </div>
  </div>
</template>

<style scoped>
.skeleton-pulse {
  background: linear-gradient(
    135deg,
    rgba(139, 92, 246, 0.03) 0%,
    rgba(139, 92, 246, 0.08) 50%,
    rgba(139, 92, 246, 0.03) 100%
  );
  background-size: 200% 200%;
  animation: shimmer 1.8s ease-in-out infinite;
  border: 1px solid rgba(139, 92, 246, 0.04);
}
@keyframes shimmer {
  0% { background-position: 0% 50%; }
  50% { background-position: 100% 50%; }
  100% { background-position: 0% 50%; }
}
</style>

<script setup lang="ts">
  import { Deferred, router } from "@inertiajs/vue3";
  import DeferredSkeleton from "../components/vue/deferred-skeleton.vue";
  import UserCard from "../components/vue/user-card.vue";

  type User = {
    id: number;
    name: string;
  };

  defineProps<{ users?: User[] }>();
  
  function handleTriggerPartialReload() {
    // basically, a reload. Could even be a form submitting to a route that returns `redirect()->back()`
    router.post("/pingpong");
  }
</script>

<template>
    <main class="main-container">
    <h1 class="mb-12 p-20 bg-amber-100">
        Listing users with <span class="underline">built-in</span> Deferred component
    </h1>

    <Deferred data="users">
        <template #fallback>
          <DeferredSkeleton />
        </template>

        <div class="flex flex-col gap-1">
            <UserCard v-for="user in users" :name="user.name" />
        </div>
    </Deferred>

    <button @click="handleTriggerPartialReload" class="mt-8 btn">
        Trigger a partial reload
    </button>
    </main>
</template>

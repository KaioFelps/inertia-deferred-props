<script lang="ts">
  import { router } from "@inertiajs/core";
  import { Deferred } from "@inertiajs/svelte";
  import DeferredSkeleton from "../components/svelte/deferred-skeleton.svelte";
  import UserCard from "../components/svelte/user-card.svelte";

    type User = {
    id: number;
    name: string;
    };

    const { users }: { users: User[] } = $props();

  function handleTriggerPartialReload() {
    // basically, a reload. Could even be a form submitting to a route that returns `redirect()->back()`
    router.post("pingpong");
  }
</script>

<main class="main-container">
  <h1 class="mb-12 p-20 bg-amber-100">
    Listing users with <span class="underline">built-in</span> Deferred component
  </h1>

  <Deferred data="users">
    {#snippet fallback()}
      <DeferredSkeleton />
    {/snippet}

    <div class="flex flex-col gap-1">
        <!-- if it was props.users?.map instead, it wouldn't throw the error and the fallback would be shown again -->
        {#each users as user}
          <UserCard name={user.name} />
        {/each}
    </div>
  </Deferred>

  <button onClick={handleTriggerPartialReload} class="mt-8 btn">
    Trigger a partial reload
  </button>
</main>

<script lang="ts">
  import "./app.css";
  import { onMount } from "svelte";

  enum State {
    NotVerified,
    LoggedIn,
  }

  interface Links {
    links: Link[],
    rf: string | null
  }

  interface Link {
    name: string;
    url: string;
  }

  let state = State.NotVerified;
  let dn: string;
  let passwd: string;
  let ref: string | null = new URL(window.location.href).searchParams.get(
    "ref"
  );
  console.log(ref);
  let links: Link[];

  onMount(async () => {
    const res = await fetch("/api/auth", {
      method: "POST",
    });
    if (res.ok) {
      if (ref) {
        window.location.href = ref;
      } else {
        state = State.LoggedIn;
        const j: Link[] | null = await res.json();
        if (j) {
          links = j;
        }
      }
    } else {
      state = State.NotVerified;
    }
  });

  const login = async () => {
    const res = await fetch("/api/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        dn: dn,
        passwd: passwd,
        rf: ref
      }),
    });
    if (res.ok) {
      state = State.LoggedIn;
      const j: Links | null = await res.json();
      if (j) {
        links = j.links;
        if (j.rf) {
          window.location.href = j.rf;
        }
      }
    }
  };
</script>

{#if state === State.NotVerified}
  <main class="mt-32">
    <div class="flex flex-col items-center">
      <div class="text-3xl">WAY</div>
      <div class="mt-2 text-sm text-stone-400">
        <a href="https://git.sr.ht/~kyoheiu/way-rs" target="_blank">v0.3.0</a>
      </div>
      <div>
        <div class="m-2 flex flex-col items-center">
          <div class="m-2">
            <input
              class="text-stone-600"
              type="text"
              bind:value={dn}
              placeholder="DN"
              required
            />
          </div>
          <div class="m-2">
            <input
              class="text-stone-600"
              type="password"
              bind:value={passwd}
              placeholder="PASSWORD"
              required
            />
          </div>
          <div>
            <button class="mt-4 bg-slate-600" on:click={() => login()}>
              &nbsp;GO&nbsp;
            </button>
          </div>
        </div>
      </div>
    </div>
  </main>
{:else}
  <main class="mt-32">
    <div class="flex flex-col items-center">
      <div class="text-3xl">How's it going?</div>
      <div class="mb-4 mt-2 text-sm text-stone-400">
        <a href="https://git.sr.ht/~kyoheiu/way-rs" target="_blank"
          >way v0.3.0</a
        >
      </div>
      <div>
        {#if links}
          {#each links as link}
            <div class="p-1">
              <a href={link.url}>
                <span class="text-lg"> > {link.name}</span>
              </a>
            </div>
          {/each}
        {/if}
      </div>
      <a href="/api/logout">
        <button class="mt-6 bg-slate-600"> &nbsp;BYE&nbsp; </button>
      </a>
    </div>
  </main>
{/if}

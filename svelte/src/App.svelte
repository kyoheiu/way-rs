<script lang="ts">
  import "./app.css";
  import { onMount } from "svelte";

  enum State {
    NotVerified,
    LoggedIn,
  }

  interface Res {
    links: Link[] | null;
    redirect: boolean;
  }

  interface Link {
    name: string;
    url: string;
  }

  let ver = "v0.3.1";
  let state = State.NotVerified;
  let dn: string;
  let passwd: string;
  let ref: string | null = new URL(window.location.href).searchParams.get(
    "ref"
  );
  let links: Link[];

  onMount(async () => {
    const res = await fetch("/api/auth");
    if (res.ok) {
      const j: Res = await res.json();
      if (j.redirect) {
        window.location.href = ref;
      } else {
        links = j.links;
        state = State.LoggedIn;
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
        redirect: ref ? true : false,
      }),
    });
    if (res.ok) {
      const j: Res = await res.json();
      if (j.redirect) {
        window.location.href = ref;
      } else {
        links = j.links;
        state = State.LoggedIn;
      }
    }
  };

  const logout = async () => {
    const _res = await fetch("/api/logout", {
      method: "POST",
    });
    state = State.NotVerified;
  };
</script>

{#if state === State.NotVerified}
  <main class="mt-32">
    <div class="flex flex-col items-center">
      <div class="text-3xl">WAY</div>
      <div class="mt-2 text-sm text-stone-400">
        <a href="https://git.sr.ht/~kyoheiu/way-rs" target="_blank">{ver}</a>
      </div>
      <div>
        <div class="m-2 flex flex-col items-center">
            <input
              class="text-stone-600 w-2/3 mt-2 mb-1"
              type="text"
              bind:value={dn}
              placeholder="DN"
              required
            />
            <input
              class="text-stone-600 w-2/3 mt-1 mb-2"
              type="password"
              bind:value={passwd}
              placeholder="PASSWORD"
              required
            />
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
      <div class="text-3xl">WAY</div>
      <div class="mb-4 mt-2 text-sm text-stone-400">
        <a href="https://git.sr.ht/~kyoheiu/way-rs" target="_blank"
          >{ver}</a
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
      <button class="mt-6 bg-slate-600" on:click={() => logout()}>
        &nbsp;BYE&nbsp;
      </button>
    </div>
  </main>
{/if}

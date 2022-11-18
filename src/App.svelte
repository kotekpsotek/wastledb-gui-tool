<script lang="ts">
  import "carbon-components-svelte/css/g10.css"; // IBM G10 white theme
  import { ToastNotification } from "carbon-components-svelte"; // IBM Carbon components // Because: they are delightful
  import { onMount } from "svelte";
  import { fly, scale } from "svelte/transition";
  import { emit, listen } from "@tauri-apps/api/event";
  import ConnectionsList from "./lib/ts/connectionsList";
  import { connectionsStore, displayingState, notificationStateStore as notification, databaseTablesList, dbsDatabasesList } from "./lib/ts/storages";
  import LeftStripeContent from "./lib/LeftStripeContent.svelte";
  import EstablishConnection from "./lib/EstablishConnection.svelte";
  import SelectedTable from "./lib/SelectedTable.svelte"

  // When program has been loaded
  onMount(async () => {
    // Read connections list when app was started
    let readyFc = await ConnectionsList.readConnections();

    // Set connections from file to storage with connections list
    connectionsStore.update(value => {
      return value = readyFc.connections;
    });
  });
  
  function notificationContent(): string {
    return $notification[1] ? $notification[1] : ("Incorrect login data try again..." as any) as string
  }

  listen("show-tables-res", ev => {
    // Show dbs tables
    console.log("tables achived!")
    const { tables } = JSON.parse(ev.payload as string) as { tables: string[] }; // parse recived payload and extract "tables" by destructurization
    console.log(tables)
    $displayingState = "table_list";
    $databaseTablesList = tables;
  });

  listen("show-databases-res", ev => {
    // Show databases from dbs
    const { databases } = JSON.parse(ev.payload as string) as { databases: string[] };
    $displayingState = "databases_list";
    $dbsDatabasesList = databases;
  });

  listen("error", ev => {
    const reason = ev.payload as string;
    $notification = [true, reason || "Error detected", false];
  })
</script>

{#if $notification[0]}
  <div class="notification" in:fly={{ duration: 200, x: 300 }} out:scale={{ duration: 200 }}>
    <ToastNotification id="app-notification-toast" lowContrast={true} timeout={10_000} title="Error" caption={new Date().toLocaleTimeString("pl-PL")} subtitle={notificationContent()} kind={$notification[2] ? "success" : "error"} on:close={ev => {$notification[0] = false; ($notification[2] ? $notification[2] = false : null)}}/>
  </div>
{/if}

<div class="left-stripe">
  {#key $displayingState}
    <LeftStripeContent/>
  {/key}
</div>

<div class="body-action">
  {#if $displayingState == "es_connection"}
    <EstablishConnection/>
  {:else if $displayingState == "table_list"}
    <SelectedTable/>
  {/if}
</div>

<style>
  :root {
    --width-left-stripe: 350px;
    --width-body-action: calc(100vw - var(var(--width-left-stripe)));
  }

  /* notification */
  .notification {
    position: absolute;
    top: 10px;
    right: 10px;
    z-index: 10 !important;
  }

  /* .Left stripe styles */
  .left-stripe {
    width: var(--width-left-stripe);
    background-color: var(--orange-hue);
    display: flex;
    flex-direction: column;
    row-gap: 5px;
    border-right: solid 3px var(--white-background);
  }

  /* Body Action styles */
  .body-action {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
</style>

<script lang="ts">
  import "carbon-components-svelte/css/g10.css"; // IBM G10 white theme
  import { ToastNotification } from "carbon-components-svelte"; // IBM Carbon components // Because: they are delightful
  import { onMount } from "svelte";
  import { fly, scale } from "svelte/transition";
  import { emit, listen } from "@tauri-apps/api/event";
  import ConnectionsList from "./lib/ts/connectionsList";
  import { connectionsStore, displayingState, notificationStateStore as notification, databaseTablesList, dbsDatabasesList, connectedToDatabaseName, selectedTableContent } from "./lib/ts/storages";
  import LeftStripeContent from "./lib/LeftStripeContent.svelte";
  import EstablishConnection from "./lib/EstablishConnection.svelte";
  import SelectedTable from "./lib/SelectedTable.svelte";
  import NoSelectedDatabase from "./lib/noSelectedDatabase.svelte";
  import type { Table as TableSchema } from "./lib/ts/utilTypes";

  // When program has been loaded
  onMount(async () => {
    // Read connections list when app was started
    let readyFc = await ConnectionsList.readConnections();

    // Set connections from file to storage with connections list
    connectionsStore.update(value => {
      return value = readyFc.connections;
    });
  });

  listen("show-tables-res", ev => {
    /* Show dbs tables */

    // Parse recived payload and extract "tables" by destructurization
    const { tables } = JSON.parse(ev.payload as string) as { tables: string[] };

    // Change app state
    $displayingState = "table_list";

    // Push tables list to tables stroage
    $databaseTablesList = tables;
  });

  listen("show-databases-res", ev => {
    /* Show databases from dbs */

    // Get databases list from JSON object
    const { databases } = JSON.parse(ev.payload as string) as { databases: string[] };

    // Change app state
    $displayingState = "databases_list";

    // Push databases list to database storage
    $dbsDatabasesList = databases;
  });

  listen("connected-to-database", async ev => {
    /* When user has been connected to another database */

    // under payload is database name in raw string format
    $connectedToDatabaseName = ev.payload as string;

    // delete selected table from previous database (regardless whether it has been prior selected or not)
    $selectedTableContent = undefined;

    // request for new connected database-name tables
    await emit("show-tables");
  });

  listen("table-content", ev => {    
    // Get content of selected table
    const table: TableSchema = JSON.parse(JSON.parse(ev.payload as string).table as string);
    
    // Push selected table content to storage
    $selectedTableContent = table;
  });

  listen("error", ev => {
    /* Display error notification */
    const reason = ev.payload as string;
    $notification = [true, reason || "Error detected", false];
  });
</script>

{#if $notification[0]}
  <div class="notification" in:fly={{ duration: 200, x: 300 }} out:scale={{ duration: 200 }}>
    <ToastNotification id="app-notification-toast" lowContrast={true} timeout={10_000} title={$notification[2] ? "Success" : "Error"} caption={new Date().toLocaleTimeString("pl-PL")} subtitle={$notification[1] ? $notification[1] : "Incorrect login data try again..."} kind={$notification[2] ? "success" : "error"} on:close={ev => {$notification[0] = false; ($notification[2] ? $notification[2] = false : null)}}/>
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
  {:else if $displayingState == "databases_list"}
    <NoSelectedDatabase/>
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

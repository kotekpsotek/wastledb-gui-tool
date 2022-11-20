<script lang="ts">
    import { List, DatabaseDatastax, Table, Datastore, Cube } from "carbon-icons-svelte" // Icon IBM Carbon
    import { connectionsStore } from "./ts/storages";
    import { onMount, afterUpdate } from "svelte";
    import { emit } from "@tauri-apps/api/event";
    import { displayingState, connectedToDatabaseName, databaseTablesList, selectedTableName, dbsDatabasesList } from "./ts/storages";

    let heigthLs3 = 0;
    
    onMount(async () => {
      const titleLs = document.querySelector(".left-stripe > .title").clientHeight;
      const tablesLsTitle = document.querySelector(".left-stripe > .tables-title").clientHeight;
      heigthLs3 = document.body.clientHeight - (titleLs + tablesLsTitle);
    });

    async function userChooseTable(ev: Event) {
      // Show selected table content
      // TODO: emit event to backend in order to advance table content or !error
      const clickedTableName = (ev.currentTarget as HTMLElement).querySelector(".table-name p").textContent.trim();
      $selectedTableName = clickedTableName

      // Download table content
      await emit("get-table-content", clickedTableName);
    }

    async function userChooseDatabase(ev: Event) {
      const databaseName = (ev.currentTarget as HTMLElement).querySelector(".database-name p").textContent;
      await emit("connect-to-database", databaseName);
    }

    afterUpdate(() => {
      // After click on database name show all databases names "go back to it" - because point from which it is emitted is further
      document.getElementById("choosed-database-name")?.addEventListener("click", async ev => {
        await emit("show-databases");
      });
    });
</script>

<div class="title">
  {#if $displayingState == "es_connection"}
    <List size={24} class="list-icon" color="white"/>
    <p>Recent Connections</p>
  {:else if $displayingState == "table_list"}
    <DatabaseDatastax size={24} class="list-icon" color="white"/>
    <p title="Connected with database {$connectedToDatabaseName}" id="choosed-database-name">{$connectedToDatabaseName}</p>
  {:else if $displayingState == "databases_list"}
    <Cube size={24} class="list-icon" color="white"/>
    <p>Available databases</p>
  {/if}
</div>
<div class="tables-title">
  {#if $displayingState == "es_connection"}
    <p>Connections List</p>
  {:else if $displayingState == "table_list"}
    <p>Tables</p>
  {:else if $displayingState == "databases_list"}
    <p>Databases</p>
  {/if}
</div>
<div class="connections-list" style={`height: ${heigthLs3}px;`}>
    {#if $displayingState == "es_connection"}
        <!-- Display connection in order: the newest connections are on top of list but older on the bottom -->
        {#each $connectionsStore.reverse() as connection, id}
          <div class="connection-outcome">
            <div class="num">
              {id + 1}
            </div>
            <div class="text">
              <div class="dbname">
                <p>{connection.connectionUrl}</p>
              </div>
              <div class="date">
                {`${new Date(connection.dateTimestamp).toLocaleDateString("pl-PL")} ${new Date(connection.dateTimestamp).toLocaleTimeString("pl-PL")}`}
              </div>
            </div>
          </div>
        {/each}
    {:else if $displayingState == "table_list"}
      {#if $databaseTablesList.length}
        {#each $databaseTablesList as table}
          <button class="table-outcome" on:click={userChooseTable}>
            <div class="tbl-icon">
              <Table size={16} color="white" id="icon-src"/>
            </div>
            <div class="table-name">
              <p>{table}</p>
            </div>
          </button>
        {/each}
      {:else}
          <div class="no-tables">
            <p>Database hasn't got any tables</p>
          </div>
      {/if}
    {:else if $displayingState == "databases_list"}
      {#if $dbsDatabasesList.length}
        {#each $dbsDatabasesList as database}
          <button class="database-outcome" on:click={userChooseDatabase}>
            <div class="db-icon">
              <Datastore size={16} color="white" id="icon-src"/>
            </div>
            <div class="database-name">
              <p>{database}</p>
            </div>
          </button>
        {/each}
      {:else}
        <div class="no-databases">
          <p>Empty databases list</p>
        </div>
      {/if}
    {/if}
</div>

<style>
    button {
      width: 100%;
      background-color: transparent;
      border: none;
      outline: none;
    }

    .no-databases, .no-tables {
      width: 100%;
      height: 100%;
      display: flex;
      align-items: center;
      justify-content: center;
      color: white;
    }

    .no-databases > p, .no-tables > p {
      font-size: 15px !important;
    }

    .title, .tables-title {
      width: 100%;
      display: flex;
      align-items: center;
      column-gap: 5px;
      padding-left: 5px;
      padding-right: 5px;
    }

    .title {
      padding-bottom: 3px;
      border-bottom: solid 1px var(--darker-white-on-orange);
      cursor: pointer;
    }

    .title #choosed-database-name:hover {
      text-decoration: underline solid white;
    }

    .left-stripe .title p {
      font-size: 24px !important;
      color: white;
    }

    .left-stripe .tables-title p {
      font-size: 20px;
      color: var(--darker-white-on-orange);
    }

    .connections-list {
      width: 100%;
      display: flex;
      flex-direction: column;
      overflow-y: auto;
    }

    .connection-outcome, .table-outcome {
      padding-top: 10px;
      padding-bottom: 10px;
    }

    .connections-list > .connection-outcome {
      width: 100%;
      display: flex;
      align-items: center;
      column-gap: 5px;
      font-size: 18px !important;
      cursor: pointer;
    }

    .connections-list > .connection-outcome:hover {
      background-color: white;
    }

    .connection-outcome .num {
      width: 35px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: white;
      border-right: solid 1px rgb(221, 219, 219);
      margin-right: 5px;
    }

    .connection-outcome .text {
      width: calc(100% - 35px);
    }

    .connection-outcome:hover .num {
      color: var(--orange-hue);
      border-right-color: rgb(180, 180, 180);
    }

    .connection-outcome .dbname {
      color: white;
    }

    .connection-outcome .dbname p {
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
    }

    .connection-outcome:hover .dbname {
      color: var(--orange-hue);
    }

    .connection-outcome .date {
      font-size: 15px !important;
      color: rgb(221, 219, 219);
    }

    .connection-outcome:hover .date {
      color: rgb(180, 180, 180);
    }

    .table-outcome, .database-outcome {
      height: 30px;
      display: flex;
      align-items: center;
      cursor: pointer;
    }

    .table-outcome:hover, .database-outcome:hover {
      background-color: white;
    }

    .tbl-icon, .db-icon {
      display: flex;
      justify-content: center;
      align-items: center;
      width: 15%;
    }

    .table-name, .database-name {
      display: flex;
      width: calc(100% - 15%);
    }

    .table-name p, .database-name p {
      color: white;
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
    }

    .table-outcome:hover .table-name p, .database-outcome:hover .database-name p {
      color: var(--orange-hue);
    }
</style>

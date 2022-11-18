<script lang="ts">
    import { List, DatabaseDatastax, Table } from "carbon-icons-svelte" // Icon IBM Carbon
    import { connectionsStore } from "./ts/storages";
    import { onMount } from "svelte";
    import type { SupportedUITypes } from "./ts/storages"
    import { displayingState, connectedToDatabaseName, databaseTablesList, selectedTableName } from "./ts/storages";

    let heigthLs3 = 0;
    
    onMount(async () => {
      const titleLs = document.querySelector(".left-stripe > .title").clientHeight;
      const tablesLsTitle = document.querySelector(".left-stripe > .tables-title").clientHeight;
      heigthLs3 = document.body.clientHeight - (titleLs + tablesLsTitle);
    });

    function userChooseTable(ev: Event) {
      // Show selected table content
      // TODO: emit event to backend in order to advance table content or !error
      const clickedTableName = (ev.currentTarget as HTMLElement).querySelector(".table-name p").textContent.trim();
      $selectedTableName = clickedTableName
    }
</script>

<div class="title">
  {#if $displayingState == "es_connection"}
    <List size={24} class="list-icon" color="white"/>
    <p>Recent Connections</p>
  {:else if $displayingState == "table_list"}
    <DatabaseDatastax size={24} class="list-icon" color="white"/>
    <p title="Connected with database {$connectedToDatabaseName}">{$connectedToDatabaseName}</p>
  {/if}
</div>
<div class="tables-title">
  {#if $displayingState == "es_connection"}
    <p>Connections List</p>
  {:else if $displayingState == "table_list"}
    <p>Tables</p>
  {/if}
</div>
<div class="connections-list" style={`height: ${heigthLs3}px;`}>
    {#if $displayingState == "es_connection"}
        {#each $connectionsStore as connection, id}
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
    {/if}
</div>

<style>
    button {
      width: 100%;
      background-color: transparent;
      border: none;
      outline: none;
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

    .table-outcome {
      height: 30px;
      display: flex;
      align-items: center;
      cursor: pointer;
    }

    .table-outcome:hover {
      background-color: white;
    }

    .tbl-icon {
      display: flex;
      justify-content: center;
      align-items: center;
      width: 15%;
    }

    .table-name {
      display: flex;
      width: calc(100% - 15%);
    }

    .table-name p {
      color: white;
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
    }

    .table-outcome:hover .table-name p {
      color: var(--orange-hue);
    }
</style>

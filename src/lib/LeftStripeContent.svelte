<script lang="ts">
    import { List } from "carbon-icons-svelte" // Icon IBM Carbon
    import { connectionsStore } from "./ts/storages";
    import { onMount } from "svelte";
    import type { SupportedUITypes } from "./ts/storages"

    export let whatToDisplay: SupportedUITypes;

    let heigthLs3 = 0;
    
    onMount(async () => {
        const titleLs = document.querySelector(".left-stripe > .title").clientHeight;
        const tablesLsTitle = document.querySelector(".left-stripe > .tables-title").clientHeight;
        heigthLs3 = document.body.clientHeight - (titleLs + tablesLsTitle);
    });
</script>

<div class="title">
  <List size={24} class="list-icon" color="white"/>
  <p>Recent Connections</p>
</div>
<div class="tables-title">
  <p>Tables</p>
</div>
<div class="connections-list" style={`height: ${heigthLs3}px;`}>
    {#if whatToDisplay == "es_connection"}
        {#each $connectionsStore as connection, id}
          <div class="outcome">
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
    {/if}
</div>

<style>
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
      border-bottom: solid 1px white;
    }

    .left-stripe .title p {
      font-size: 24px !important;
      color: white;
    }

    .left-stripe .tables-title p {
      font-size: 20px;
      color: var(--yellow-hue);
    }

    .connections-list {
      width: 100%;
      display: flex;
      flex-direction: column;
      overflow-y: auto;
    }

    .connections-list > .outcome {
      width: 100%;
      display: flex;
      align-items: center;
      column-gap: 5px;
      font-size: 18px !important;
      cursor: pointer;
      padding-top: 5px;
      padding-bottom: 5px;
    }

    .connections-list > .outcome:hover {
      background-color: white;
    }

    .outcome .num {
      width: 35px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: white;
      border-right: solid 1px rgb(221, 219, 219);
      margin-right: 5px;
    }

    .outcome .text {
      width: calc(100% - 35px);
    }

    .outcome:hover .num {
      color: var(--orange-hue);
      border-right-color: rgb(180, 180, 180);
    }

    .outcome .dbname {
      color: white;
    }

    .dbname p {
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
    }

    .outcome:hover .dbname {
      color: var(--orange-hue);
    }

    .outcome .date {
      font-size: 15px !important;
      color: rgb(221, 219, 219);
    }

    .outcome:hover .date {
      color: rgb(180, 180, 180);
    }
</style>

<script lang="ts">
    import { selectedTableContent, notificationStateStore } from "./ts/storages";
    import { Sql } from "carbon-icons-svelte";
    import { TextInput, Button } from "carbon-components-svelte";
    import { emit, listen } from "@tauri-apps/api/event";

    // Allow to highlight number cell data
    const nullHiglight = (value: any) => value == null ? true : false;
    const numberLinting = (column_name: string) => ["INT", "FLOAT"].includes($selectedTableContent.columns.find(val => (val.name == column_name) ? true : false).d_type as string);
    const stringTypeHiglight = (column_name: any) => {
        const foundCoulmnType = $selectedTableContent.columns.find(val => (val.name == column_name) ? true : false).d_type;
        return ["TEXT", "LONGTEXT"].includes(foundCoulmnType as string) || (typeof foundCoulmnType == "object" && Object.keys(foundCoulmnType).includes("VARCHAR")) ? true : false;
    }

    // Value from input
    let inputContent: string;

    // Assign value to variable "inputContent"
    function assignValueFromInput(ev: CustomEvent) {
        inputContent = ev.detail as string;
    }

    // Execute SQL query
    async function executeSQLQueryOnTable(ev: Event) {
        // Replace 'here' to actual selecte table content
        const hereStatementToAcutualDatabaseName = inputContent.includes("here") ? inputContent.replace("here", $selectedTableContent.name) : inputContent;

        // Emit event to backend in order to execute query
        await emit("execute-sql-query", JSON.stringify({ query: hereStatementToAcutualDatabaseName, execute_on_here: inputContent.includes("here") }))
    }

    listen("execute-sql-query-successoutput", async ev => {
        /* Recive SQL query result when Query has been successfull executed (performed only in that situation) */
        const { result, execute_on_same_table }: { result: string, execute_on_same_table: boolean } = ev.payload as any; // { result: "executed_query_result", execute_on_same_table: boolean }

        // Display notification
        // Show performed 
        if (result != "query_performed") {
            // Display result of executed query
            $notificationStateStore[0] = true;
            $notificationStateStore[1] = result;
            $notificationStateStore[2] = true;
        }
        else {
            // When database doesn't attach query execution result display basic information about that
            $notificationStateStore[0] = true;
            $notificationStateStore[1] = "Query has been executed!";
            $notificationStateStore[2] = true;
        }

        // When query has been executed on same table then reload it
        if (execute_on_same_table) {
            await emit("get-table-content", $selectedTableContent.name);
        }
    });
</script>

{#if $selectedTableContent}
    <div class="selected-table">
        <div class="title-sql-execution">
            <div class="title">
                <p>{$selectedTableContent.name}</p>
            </div>
            <div class="sql-linter">
                <Button id="sql-execute-button" size="small" kind="tertiary" on:click={executeSQLQueryOnTable}>
                    <Sql size={16}/>
                </Button>
                <TextInput id="sql-input-linter" placeholder="Enter SQL Query to execute it..." on:input={assignValueFromInput}/>
            </div>
        </div>
        <div class="src">
            <table class="content">
                <thead>
                    <tr>
                        {#each $selectedTableContent.columns as { name, d_type }}
                            <th>
                                <p class="table-name">{name}</p>
                                <p class="data-type">
                                    {#if typeof d_type == "object" && !(d_type instanceof Array)}
                                        <!-- Support for types with defined properties (these data-types must be object types) -->
                                        {#if Object.keys(d_type).includes("VARCHAR")}
                                            <!-- Support for "Varchar" type -->
                                            varchar(<span class="number-into-type" title="{d_type["VARCHAR"] ? String(d_type["VARCHAR"]) : String(65535)}">{d_type["VARCHAR"] || "max"}</span>)
                                        {/if}
                                    {:else}
                                        {d_type}
                                    {/if}
                                </p>
                            </th>
                        {/each}
                    </tr>
                </thead>
                <tbody>
                    {#if $selectedTableContent.rows?.length}
                        {#each $selectedTableContent.rows as row}
                            <tr>
                                {#each row as { value, col: column_name }}
                                    {#if nullHiglight(value)}
                                        <!-- Higlight null value -->
                                        <td class="null">{value}</td>
                                    {:else}
                                        {#if numberLinting(column_name)}
                                            <!-- Higlight number value -->
                                            <td class="number">{value}</td>
                                        {:else if stringTypeHiglight(column_name)}
                                            <td class="string">{value}</td>
                                        {:else}
                                            <td>{value}</td>
                                        {/if}
                                    {/if}
                                {/each}
                            </tr>
                        {/each}
                    {:else}
                        <!-- When table is without records -->
                        <tr class="table-without-content">
                            {#each $selectedTableContent.columns as _}
                                <td></td>
                            {/each}
                        </tr>
                    {/if}
                </tbody>
            </table>
        </div>
    </div>
{:else}
    <div class="table-doesnt-selected">
        <p>Select table from list to display it content!</p>
    </div>
{/if}

<style>
    /* When user doesn't choose any table */
    .table-doesnt-selected > p {
        font-size: 20px;
    }

    /* When user choose some table */
    .selected-table {
        width: 98%;
        height: 100%;
        padding-top: 2px;
        display: flex;
        flex-direction: column;
        row-gap: 10px;
    }

    .title-sql-execution {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .title-sql-execution .title p {
        font-size: 28px;
    }

    .title-sql-execution .sql-linter {
        display: flex;
    }

    .selected-table .src {
        padding: 1px;
        width: 100%;
        height: 100%;
        overflow: auto;
    }

    table.content {
        width: 100%;
        empty-cells: show;
        background-color: var(--white-background);
        border: solid 1px var(--orange-hue);
        border-radius: 2px;
    }

    table.content thead th {
        padding: 10px;
        padding-bottom: 3px;
        padding-top: 3px;
    }

    table.content th .table-name {
        font-size: 17px;
        color: white;
    }

    /* Higlight column data-type  */
    table.content th .data-type {
        color: var(--orange-hue);
    }

    /* Higlight column data type property when it exists */
    table.content th .data-type span.number-into-type {
        cursor: pointer;
        color: greenyellow;
    }
    
    table.content td {
        padding: 5px;
    }

    table.content tbody tr td {
        border-right: 1px solid black;
    }

    table.content tbody tr.table-without-content td {
        border-right: none !important;
    }

    table.content tbody tr td:last-of-type {
        border-right: none;
    }

    /* Linting for specific data type */
    .number {
        color: greenyellow;
    }

    .null {
        color: blue;
    }

    .string {
        color: orangered;
    }
</style>

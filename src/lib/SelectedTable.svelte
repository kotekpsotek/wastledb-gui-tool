<script lang="ts">
    import { selectedTableName } from "./ts/storages";
    import { Sql } from "carbon-icons-svelte";
    import { TextInput, Button } from "carbon-components-svelte";
</script>

{#if $selectedTableName}
    <div class="selected-table">
        <div class="title-sql-execution">
            <div class="title">
                <p>{$selectedTableName}</p>
            </div>
            <div class="sql-linter">
                <Button id="sql-execute-button" size="small" kind="tertiary">
                    <Sql size={16}/>
                </Button>
                <TextInput id="sql-input-linter" placeholder="Enter SQL Query to execute it..."/>
            </div>
        </div>
        <div class="src">
            <table class="content">
                <tr>
                    <th>Column name 1</th>
                    <th>Column name 2</th>
                    <th>Column name 3</th>
                </tr>
                <tr>
                    <td>row value</td>
                    <td>row value</td>
                    <td>row value</td>
                </tr>
                <tr>
                    <td>row value</td>
                    <td>row value</td>
                    <td>row value</td>
                </tr>
                <tr>
                    <td>row value</td>
                    <td>row value</td>
                    <td>row value</td>
                </tr>
                <tr>
                    <td>row value</td>
                    <td>row value</td>
                    <td>row value</td>
                </tr>
                <tr>
                    <td>row value</td>
                    <td>row value</td>
                    <td>row value</td>
                </tr>
                <tr>
                    <td>row value</td>
                    <td>row value</td>
                    <td>row value</td>
                </tr>
                <tr>
                    <td>row value</td>
                    <td>row value</td>
                    <td>row value</td>
                </tr>
                <tr>
                    <td>row value</td>
                    <td>row value</td>
                    <td>row value</td>
                </tr>
                <tr>
                    <td>row value</td>
                    <td>row value</td>
                    <td>row value</td>
                </tr>
                <tr>
                    <td>row value</td>
                    <td>row value</td>
                    <td>row value</td>
                </tr>
                <tr>
                    <td>row value</td>
                    <td>row value</td>
                    <td>row value</td>
                </tr>
                <tr>
                    <td>row value</td>
                    <td>row value</td>
                    <td>row value</td>
                </tr>
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

    table.content > tr td {
        border-right: 1px solid black;
    }

    table.content > tr td:last-of-type {
        border-right: none;
    }

    table.content th {
        padding: 10px;
        font-size: 17px;
        color: white;
    }
    
    table.content td {
        padding: 5px;
    }
</style>

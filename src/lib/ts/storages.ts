import { writable } from "svelte/store";
import type { connectionOne } from "./connectionsList";
import type { Table } from "./utilTypes";

export const connectionsStore = writable<connectionOne[]>([]);

export type SupportedUITypes = "es_connection" | "databases_list" | "table_list";
export const displayingState = writable<SupportedUITypes>("es_connection");

export const notificationStateStore = writable<[boolean, string, boolean]>([false, "", false]);

export const databaseTablesList = writable<string[]>([]);
export const dbsDatabasesList = writable<string[]>([]);
export const connectedToDatabaseName = writable<string>();
export const selectedTableName = writable<string>();
export const selectedTableContent = writable<Table>(); // table content in object type obtained from JSON object

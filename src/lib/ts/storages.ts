import { writable } from "svelte/store";
import type { connectionOne } from "./utilTypes";
import type { Table, SupportedUITypes } from "./utilTypes";

// What application should display state
export const displayingState = writable<SupportedUITypes>("es_connection");
// Whether app should display notification and it content
export const notificationStateStore = writable<[boolean, string, boolean]>([false, "", false]);
// Lists with contents for specific displaying states
export const connectionsStore = writable<connectionOne[]>([]);
export const databaseTablesList = writable<string[]>([]);
export const dbsDatabasesList = writable<string[]>([]);
export const connectedToDatabaseName = writable<string>();
export const selectedTableContent = writable<Table>(); // table content in object type obtained from JSON object

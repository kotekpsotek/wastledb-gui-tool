import { writable } from "svelte/store";
import type { connectionOne } from "./connectionsList";

export const connectionsStore = writable<connectionOne[]>([]);

export type SupportedUITypes = "es_connection" | "databases_list" | "table_list";
export const displayingState = writable<SupportedUITypes>("es_connection");

export const notificationStateStore = writable<[boolean, string, boolean]>([false, "", false]);

export const databaseTablesList = writable<string[]>([]);
export const dbsDatabasesList = writable<string[]>([]);
export const connectedToDatabaseName = writable<string>();
export const selectedTableName = writable<string>();

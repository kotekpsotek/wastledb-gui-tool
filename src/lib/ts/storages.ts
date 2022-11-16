import { writable } from "svelte/store";
import type { connectionOne } from "./connectionsList";

export const connectionsStore = writable<connectionOne[]>([])

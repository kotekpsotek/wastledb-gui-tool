/* Table from database shema */
export interface Table {
    name: string,
    columns: Array<{ constraints: string[], d_type: any, name: string }>,
    rows: Array<{ col: string, value: string }>
}

/* Types determining what can be displaying in APP */
export type SupportedUITypes = "es_connection" | "databases_list" | "table_list";

/* Connection from file with connections */
export interface connectionOne {
    id: string,
    connectionUrl: string,
    userName: string,
    dateTimestamp: number,
    databaseName?: string,
    rsaPublicKey?: string
}

/* Structure of file with connections list */
export interface connectionsList {
    connections: connectionOne[]
}

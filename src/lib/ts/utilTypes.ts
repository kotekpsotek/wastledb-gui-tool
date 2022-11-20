/* Supported datatypes span for all table columns */
type ColumnDType = { VARCHAR: Number | null } | "INT" | "FLOAT" | "TEXT" | "LONGTEXT" | "DATE" | "DATETIMESTAMP" | "NULL" | "BOOLEAN";
/* Table from database shema */
export interface Table {
    name: string,
    columns: Array<{ constraints: string[], d_type: ColumnDType, name: string }>,
    rows: Array<Array<{ col: string, value: string }>> | null // null - when table doesn't have records
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

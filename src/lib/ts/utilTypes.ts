export interface Table {
    name: string,
    columns: Array<{ constraints: string[], d_type: any, name: string }>,
    rows: Array<{ col: string, value: string }>
}

import { readTextFile, writeFile, readDir, createDir, exists } from "@tauri-apps/api/fs";
import { v4 } from "uuid";
import { connectionsStore } from "./storages";
import type { connectionsList } from "./utilTypes";

export default class connectionListOpeartions {
    static fileName = "recentConnectionsList.json";

    public static async saveConnection(connectionUrl: string, userName: string, databaseName?: string, rsaPublicKey?: string) {
        // Get connections file content and append to it new created connection content
        const actualConnections = await connectionListOpeartions.readConnections();
        const connectionObject = {
            id: v4(), // unique id for connection
            connectionUrl,
            userName,
            dateTimestamp: new Date().getTime(),
            databaseName,
            rsaPublicKey,
        };
        actualConnections.connections.push(connectionObject);

        // Update storage with connection about new connection
        connectionsStore.update(value => {
            value.push(connectionObject);
            return value
        })

        // Update file with connections about new connection
        await writeFile(connectionListOpeartions.fileName, JSON.stringify(actualConnections))
    }

    public static async readConnections(): Promise<connectionsList> {
        // Read file content and parse
        if (await exists(connectionListOpeartions.fileName)) {
            const fileContent = await readTextFile(connectionListOpeartions.fileName);
            const readyFcPre = fileContent.length ? JSON.parse(fileContent) : { connections: [] };
            const readyFc: connectionsList = "connections" in readyFcPre ? readyFcPre : { connections: [] }
    
            // Return parsed file content
            return readyFc;
        }
        else {
            return { connections: [] }
        };
    }
}
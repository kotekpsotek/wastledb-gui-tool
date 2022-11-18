<script lang="ts">
    import { TextInput, PasswordInput, Checkbox, Button } from "carbon-components-svelte"; // IBM Carbon components // Because: they are delightful
    import { emit, listen } from "@tauri-apps/api/event";
    import ConnectionsList from "./ts/connectionsList";
    import { notificationStateStore as notification, connectedToDatabaseName } from "./ts/storages";

      // store data for establish connection with database
    type valueFromInputs = string | number;
    interface connectObjType {
      serverUrl: valueFromInputs,
      userName: valueFromInputs,
      userPassword: valueFromInputs,
      databaseName: valueFromInputs,
      createEncryptedConnection: boolean,
      rsapublicKey: valueFromInputs
    } 

    // Store data for enable establish connection
    const connectionObj: connectObjType = {
      serverUrl: undefined,
      userName: undefined,
      userPassword: undefined,
      databaseName: undefined,
      createEncryptedConnection: false,
      rsapublicKey: undefined,
    }
    
    // Not inside "invalidInputsState" due to svelte reactivity behaviour
    function invalidInputsStateSetAll(incorrect: boolean, withoutSetForKey?: string[]) {
        for (const keyName of Object.keys(invalidInputsState)) {
            if (incorrect && !withoutSetForKey?.includes(keyName) && (keyName != "rsapublicKey" || keyName == "rsapublicKey" && connectionObj.createEncryptedConnection)) {
                invalidInputsState[keyName] = true
            }
            else if (connectionObj[keyName] || (keyName == "rsapublicKey" && !connectionObj["rsapublicKey"] && !connectionObj["createEncryptedConnection"])) {
                invalidInputsState[keyName] = false
            }
        };

        if (withoutSetForKey) {
            for (const keyWthName of withoutSetForKey) {
                invalidInputsState[keyWthName] = !incorrect
            }
        }
    }

    // Store data about inut state
    const invalidInputsState = {
        serverUrl: false,
        userName: false,
        databaseName: false,
        userPassword: false,
        rsapublicKey: false
    }

    // Data from inputs
    let inputPasswordAssign = (ev: Event) => {
        connectionObj.userPassword = (ev.target as any).value as string;
    }

    // Show and hide textInput field for RSA public key to encrypt connection
    function encryptedConnection() {
        connectionObj.createEncryptedConnection = !connectionObj.createEncryptedConnection;

        if (!connectionObj.createEncryptedConnection) connectionObj.rsapublicKey = undefined;
    }

    // After click on "Connect" button GUI client will be try establish connection with database
    async function connectUser(ev: Event) {
        const { serverUrl, userName, userPassword, createEncryptedConnection, rsapublicKey } = connectionObj;
        
        if (serverUrl && userName && userPassword && (!createEncryptedConnection || (createEncryptedConnection && rsapublicKey))) {
          await emit("establish-connection", connectionObj)
        }
        else {
          // When required data isn't entered
          const notFor = [];
          createEncryptedConnection && rsapublicKey ? notFor.push("rsapublicKey") : null;

          invalidInputsStateSetAll(true, notFor)
        }
    }

    // Listen events from backend
    listen("couldnt-establish-connection", ev => {
        // Couldnt establish connection
        const { payload } = ev;
        switch (payload) {
            case "you must attach login options":
                $notification[0] = true;
                $notification[1] = "You must provide correct data for these fields if you want to establish connection";
                invalidInputsStateSetAll(true);
            break;   

            case "connection url is incorrect":
                $notification[0] = true;
                $notification[1] = "You must give correct format of url. Example: wastledb://127.0.0.1:27000/database_name";
                invalidInputsStateSetAll(true, ["userName", "userPassword", "databaseName", "rsapublicKey"]);
            break;

            case "couldnt connect with dbs":
                $notification[0] = true;
                $notification[1] = "Couldn't connect with database maybe IP adress is in incorrect format or you pass bad address. Adequate URL should look similar to: wastledb://127.0.0.1:27000/database_name";
                invalidInputsStateSetAll(true, ["userName", "userPassword", "databaseName", "rsapublicKey"]);
            break;

            case "unsupported response":
                $notification[0] = true;
                $notification[1] = "Recived response which couldn't be handled!";
                invalidInputsStateSetAll(true, ["userName", "userPassword", "databaseName", "rsapublicKey"]);
            break;   
            
            case "incorrect login":
                $notification[0] = true;
                $notification[1] = "You pass incorrect login data (password or user name)";
                invalidInputsStateSetAll(true, ["serverUrl", "rsapublicKey", "databaseName"]);
            break;      
  
            default:
                $notification[0] = true;
                if ((payload as string)?.includes("couldn't connect from this reason")) {
                    const reason = (payload as string).split(":")[1];
                    $notification[1] = "Couldn't grow connection from reason: " + reason;
                }
        }
    });

    listen("connection-acquired", async ev => {
        // Connection established
        $notification[0] = true;
        $notification[1] = "Established connection with database";
        $notification[2] = true;
        invalidInputsStateSetAll(false);

        // Save connection on file with connections list
        const connectionObj1 = (connectionObj as unknown) as { [id: string]: string };
        ConnectionsList.saveConnection(connectionObj1.serverUrl, connectionObj1.userName, connectionObj1.databaseName, connectionObj1.rsapublicKey)
    
        // 
        const ipcResponse: { connected_with_database: boolean, database_name: string } | null = ev.payload ? JSON.parse(ev.payload as string) : null;
        if (ipcResponse) {
            switch(ipcResponse.connected_with_database) {
                case true:
                    // Show tables from connected database
                    await emit("show-tables");
                    $connectedToDatabaseName = ipcResponse.database_name; // assign database name
                break;

                case false:
                    // Show databases list from dbs
                    await emit("show-databases");
                break;
            }
        }
    });
</script>

<div class="establish-connection">
    <div class="sl1">
      <div class="title">
        <p>Establish Connection</p>
      </div>
      <div class="server-url-cont input-cnt">
        <TextInput placeholder="Server URL..." 
          on:input={
            ev => {
              connectionObj.serverUrl = ev.detail
            }
          } 
          invalid={invalidInputsState.serverUrl}
        />
      </div>
      <div class="user-inputs">
          <div class="authorization-data">
            <div class="title">
              <p>Authorization</p>
            </div>
            <div class="user-name-cont input-cnt">
              <TextInput placeholder="User Name..." 
                on:input={
                  ev => {
                    connectionObj.userName = ev.detail
                  }
                }
                invalid={invalidInputsState.userName}
              />
            </div>
            <div class="user-password-cont input-cnt">
              <PasswordInput id=password-input placeholder="User Password..." 
                on:input={inputPasswordAssign}
                invalid={invalidInputsState.userPassword}  
              />
            </div>
          </div>
          <div class="optional">
            <div class="title">
              <p>Optional</p>
            </div>
            <div class="user-name-cont input-cnt">
              <TextInput id="user-name" placeholder="Database Name..." color="rgb(0, 0, 0)"   
                on:input={
                  ev => {
                    connectionObj.databaseName = ev.detail
                  }
                }
                invalid={invalidInputsState.databaseName}
              />
            </div>
            <div class="encrypted-connection">
              <Checkbox labelText="Create Encrypted Connection" on:change={encryptedConnection}/>
              {#if connectionObj.createEncryptedConnection}
                <TextInput placeholder="RSA Public Key..." labelText="Database RSA public key" 
                  on:input={
                    ev => {
                      connectionObj.rsapublicKey = ev.detail
                    }
                  }
                  invalid={invalidInputsState.rsapublicKey}
                />
              {/if}
            </div>
          </div>
      </div>
    </div>
    <div class="buttons">
        <Button id="connect-button" size="field" on:click={connectUser}>Connect</Button>
    </div>
</div>

<style>
    .establish-connection {
        padding: 5px;
        padding-left: 10px;
        padding-right: 10px;
        width: 750px;
        height: 350px;
        background-color: rgb(203, 203, 203);
        box-shadow: 0px 0px 10px rgb(203, 203, 203);
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        row-gap: 5px;
        border: solid 2px var(--orange-hue);
        border-radius: 2px;
    }

    .establish-connection > .sl1 {
        display: flex;
        flex-direction: column;
        row-gap: 7px;
    }

    .establish-connection > .sl1 > .title {
        display: flex;
        justify-content: center;
    }

    .establish-connection > .sl1 > .title > p {
        font-size: 24px !important;
        color: var(--orange-hue);
    }

    .establish-connection .user-inputs {
        width: 100%;
        display: flex;
        justify-content: space-between;
    }

    .user-inputs > div {
        width: 48%;
        display: flex;
        flex-direction: column;
    }

    .user-inputs > div > * {
        margin-bottom: 5px;
    }

    .buttons {
        display: flex;
        justify-content: flex-end;
    }
</style>

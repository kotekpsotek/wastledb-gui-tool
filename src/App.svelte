<script lang="ts">
  import { List } from "carbon-icons-svelte" // Icond
  import "carbon-components-svelte/css/g10.css"; // IBM G10 white theme
  import { TextInput, PasswordInput, Checkbox, Button } from "carbon-components-svelte"; // IBM Carbon components // Because: they are delightful
  import { onMount } from "svelte";
  let createEncryptedConnection = false;
  let heigthLs3 = 0;

  onMount(() => {
    const titleLs = document.querySelector(".left-stripe > .title").clientHeight;
    const tablesLsTitle = document.querySelector(".left-stripe > .tables-title").clientHeight;
    heigthLs3 = document.body.clientHeight - (titleLs + tablesLsTitle);
  });

  // Data from inputs
  type valueFromInputs = string | number;
  let userPassword: valueFromInputs;
  let inputPasswordAssign = (ev: Event) => {
    userPassword = (ev.target as any).value as string;
  }
  let serverUrl: valueFromInputs;
  let userName: valueFromInputs;
  let databaseName: valueFromInputs;
  let rsapublicKey: valueFromInputs;

  // Show and hide textInput field for RSA public key to encrypt connection
  function encryptedConnection() {
    createEncryptedConnection = !createEncryptedConnection;
  }
  function connectUser(ev: Event) {
    console.log(serverUrl, userPassword, userName, databaseName, rsapublicKey)
  }
</script>

<div class="left-stripe">
  <div class="title">
    <List size={24} class="list-icon" color="white"/>
    <p>Recent Connections</p>
  </div>
  <div class="tables-title">
    <p>Tables</p>
  </div>
  <div class="connections-list" style={`height: ${heigthLs3}px;`}>
    <div class="outcome">
      <div class="num">
        1
      </div>
      <div class="text">
        <div class="dbname">
          <p>Database name</p>
        </div>
        <div class="date">
          {new Date().toLocaleDateString("pl-PL")}
        </div>
      </div>
    </div>
  </div>
</div>

<div class="body-action">
  <div class="establish-connection">
    <div class="sl1">
      <div class="title">
        <p>Establish Connection</p>
      </div>
      <div class="server-url-cont input-cnt">
        <TextInput placeholder="Server URL..." on:input={ev => {
          serverUrl = ev.detail
        }}/>
      </div>
      <div class="user-inputs">
          <div class="authorization-data">
            <div class="title">
              <p>Authorization</p>
            </div>
            <div class="user-name-cont input-cnt">
              <TextInput placeholder="User Name..." on:input={ev => {
                userName = ev.detail
              }}/>
            </div>
            <div class="user-password-cont input-cnt">
              <PasswordInput id=password-input placeholder="User Password..." on:input={inputPasswordAssign}/>
            </div>
          </div>
          <div class="optional">
            <div class="title">
              <p>Optional</p>
            </div>
            <div class="user-name-cont input-cnt">
              <TextInput id="user-name" placeholder="Database Name..." color="rgb(0, 0, 0)" on:input={ev => {
                databaseName = ev.detail
              }}/>
            </div>
            <div class="encrypted-connection">
              <Checkbox labelText="Create Encrypted Connection" on:change={encryptedConnection}/>
              {#if createEncryptedConnection}
                <TextInput placeholder="RSA Public Key..." labelText="Database RSA public key" on:input={ev => {
                  rsapublicKey = ev.detail
                }}/>
              {/if}
            </div>
          </div>
      </div>
    </div>
    <div class="buttons">
        <Button id="connect-button" size="field" on:click={connectUser}>Connect</Button>
    </div>
  </div>
</div>

<style>
  :root {
    --width-left-stripe: 350px;
    --width-body-action: calc(100vw - var(var(--width-left-stripe)));
  }

  /* .Left stripe styles */
  .left-stripe {
    width: var(--width-left-stripe);
    background-color: var(--orange-hue);
    display: flex;
    flex-direction: column;
    row-gap: 5px;
  }

  .left-stripe .title, .left-stripe .tables-title {
    width: 100%;
    display: flex;
    align-items: center;
    column-gap: 5px;
    padding-left: 5px;
    padding-right: 5px;
  }

  .left-stripe .title {
    padding-bottom: 3px;
    border-bottom: solid 1px white;
  }

  .left-stripe .title p {
    font-size: 24px !important;
    color: white;
  }

  .left-stripe .tables-title p {
    font-size: 20px;
    color: var(--yellow-hue);
  }

  .left-stripe .connections-list {
    width: 100%;
    display: flex;
    flex-direction: column;
    overflow-y: auto;
  }

  .connections-list > .outcome {
    width: 100%;
    display: flex;
    align-items: center;
    column-gap: 5px;
    font-size: 18px !important;
    cursor: pointer;
    padding-top: 5px;
    padding-bottom: 5px;
  }

  .connections-list > .outcome:hover {
    background-color: white;
  }

  .outcome .num {
    width: 35px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    border-right: solid 1px rgb(221, 219, 219);
    margin-right: 5px;
  }

  .outcome:hover .num {
    color: var(--orange-hue);
    border-right-color: rgb(180, 180, 180);
  }

  .outcome .dbname {
    color: white;
  }

  .outcome:hover .dbname {
    color: var(--orange-hue);
  }

  .outcome .date {
    font-size: 15px !important;
    color: rgb(221, 219, 219);
  }

  .outcome:hover .date {
    color: rgb(180, 180, 180);
  }

  /* Body Action styles */
  .body-action {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  .body-action .establish-connection {
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

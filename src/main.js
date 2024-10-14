const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

function config(){
  invoke('config_setup')
  invoke('start').then(loot => {document.querySelector("#loot").textContent = loot})
}
window.addEventListener("DOMContentLoaded", () => {
  invoke('start').then(loot => {document.querySelector("#loot").textContent = loot})
});

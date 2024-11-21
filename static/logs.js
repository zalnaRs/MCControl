const startButton = document.querySelector("#start_btn");
const stopButton = document.querySelector("#stop_btn");
const killButton = document.querySelector("#kill_btn");

class Actions {
    constructor() {
        startButton.addEventListener("click", async () => {
            await fetch(window.API_URL.start.url, {method: window.API_URL.start.method})
        })
        stopButton.addEventListener("click", async () => {
            await fetch(window.API_URL.stop.url, {method: window.API_URL.stop.method})
        })
        killButton.addEventListener("click", async () => {
            await fetch(window.API_URL.kill.url, {method: window.API_URL.kill.method})
        })
    }
}

export default Actions
const startButton = document.querySelector("#start_btn");
const stopButton = document.querySelector("#stop_btn");
const killButton = document.querySelector("#kill_btn");

class Actions {
    constructor() {
        startButton.addEventListener("click", async () => {
            await fetch(window.API_URL.start.url, {
                method: window.API_URL.start.method,
                headers: {'Authorization': `Bearer ${window.password}`}
            })
        })
        stopButton.addEventListener("click", async () => {
            if (confirm("Are you sure you want to stop the process?")) {
                await fetch(window.API_URL.stop.url, {
                    method: window.API_URL.stop.method,
                    headers: {'Authorization': `Bearer ${window.password}`}
                })
            }
        })
        killButton.addEventListener("click", async () => {
            if (confirm("Are you sure you want to kill the process?")) {
                await fetch(window.API_URL.kill.url, {
                    method: window.API_URL.kill.method,
                    headers: {'Authorization': `Bearer ${window.password}`}
                })
            }
        })
    }
}

export default Actions
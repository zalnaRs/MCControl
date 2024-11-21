const logText = document.querySelector("#log_text")

class Logs {
    constructor() {
    }

    async refresh() {
        logText.innerHTML = await (await fetch(window.API_URL.logs.url)).text()
    }
}

export default Logs
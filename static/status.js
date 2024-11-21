const statusText = document.querySelector("#status_text")
const statusRefreshButton = document.querySelector("#status_btn")

class Status {
    constructor() {
    }

    async refresh() {
        statusText.innerHTML = await (await fetch(window.API_URL.status.url, {
            headers: {
                'Authorization': `Bearer ${window.password}`
            }
        })).text()
    }
}

export default Status
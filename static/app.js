const BASE_URL = "/api/";
window.API_URL = {
    status: {url: `${BASE_URL}status`, method: "GET"},
    execute: {url: `${BASE_URL}execute`, method: "POST"},
    start: {url: `${BASE_URL}start`, method: "POST"},
    stop: {url: `${BASE_URL}stop`, method: "POST"},
    kill: {url: `${BASE_URL}kill`, method: "POST"},
    logs: {url: `${BASE_URL}logs`, method: "GET"},
};

window.onload = async () => {
    window.password = prompt("Enter password:");
    const refreshIntervalInput = document.querySelector("#refresh_interval_input")

    const Status = new (await import("/static/status.js")).default()
    const Actions = new (await import("/static/actions.js")).default()
    const Logs = new (await import("/static/logs.js")).default()
    const Commands = new (await import("/static/commands.js")).default()

    const setRefreshInterval = () => {
        clearInterval(window.refreshInterval);
        window.refreshInterval = setInterval(() => {
            Status.refresh();
            Logs.refresh();
        }, parseInt(refreshIntervalInput.value));
    };

    setRefreshInterval();
    refreshIntervalInput.onchange = setRefreshInterval;
}
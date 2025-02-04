const BASE_URL = "./api/";
window.API_URL = {
    status: {url: `${BASE_URL}status`, method: "GET"},
    execute: {url: `${BASE_URL}execute`, method: "POST"},
    start: {url: `${BASE_URL}start`, method: "POST"},
    stop: {url: `${BASE_URL}stop`, method: "POST"},
    kill: {url: `${BASE_URL}kill`, method: "POST"},
    logs: {url: `${BASE_URL}logs`, method: "GET"},
};

window.onload = async () => {
    const mainView = document.querySelector("#main_view")
    const loginView = document.querySelector("#login_view")

    mainView.style.display = "none"

    const loginForm = document.querySelector("#login_form")
    const passwordInput = document.querySelector("#login_form_password")

    loginForm.addEventListener("submit", async (e) => {
        e.preventDefault()
        e.stopPropagation()
        window.password = passwordInput.value
        mainView.style.display = "block"
        loginView.style.display = "none"
        await connect()
    })

    const connect = async () => {
        const refreshIntervalInput = document.querySelector("#refresh_interval_input")

        const Status = new (await import("./status.js")).default()
        const Actions = new (await import("./actions.js")).default()
        const Logs = new (await import("./logs.js")).default()
        const Commands = new (await import("./commands.js")).default()

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
}

const commandForm = document.querySelector("#command_form")
const commandInput = document.querySelector("#command_input")

class Commands {
    constructor() {
        commandForm.addEventListener("submit", async (e) => {
            e.preventDefault()
            await fetch(window.API_URL.execute.url, {
                method: window.API_URL.execute.method,
                body: JSON.stringify({command: commandInput.value}),
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${window.password}`
                }
            })

            commandInput.value = ""
        })
    }
}

export default Commands
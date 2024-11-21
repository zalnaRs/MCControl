const logText = document.querySelector("#log_text")
const scrollToBottomCheckbox = document.querySelector("#scroll_to_bottom_checkbox")
import {AnsiUp} from './ansi_up.js'

const ansi_up = new AnsiUp();

class Logs {
    constructor() {
    }

    async refresh() {

        const log = await (await fetch(window.API_URL.logs.url, {
            headers: {

                'Authorization': `Bearer ${window.password}`
            }
        })).text()

        logText.innerHTML = ansi_up.ansi_to_html(log);

        if (scrollToBottomCheckbox.checked) {
            logText.scrollTop = logText.scrollHeight;
        }
    }
}

export default Logs
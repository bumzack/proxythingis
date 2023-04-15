
$(document).ready(function () {

    const url = "http://www.proxythingi.at/proxythingi/server"
    $.ajax({
        url: url, async: false, success: function (output) {
            // $(selector).html (output)
            console.log(`servers ${JSON.stringify(output, null, 4)}`);
        }
    });

    console.log("yeahhhh!");
});
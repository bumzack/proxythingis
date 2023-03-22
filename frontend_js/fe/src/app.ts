import jquery from "jquery";


// https://isotropic.co/how-to-fix-the-property-does-not-exist-on-type-window-error-in-typescript/
declare global {
    interface Window { // ⚠️ notice that "Window" is capitalized here
        $: any;
        jquery: any;
    }
}

window.$ = window.jquery = jquery;

jquery(document).ready(function () {
    console.log("ready!");

    jquery("#btnLoad").click((event: jquery.Event) => {
        console.log(`got a click  ${JSON.stringify(event)}`);

        // jquery.get("http://127.0.0.1:3036/api/person", function (data) {
        //     console.log(`data: ${JSON.stringify(data, null, 4)}`)
        //     jquery("#persons").html(JSON.stringify(data, null, 4));
        // });

        let person = {
            firstname: "Bli",
            lastname:"Blupp"
        }

        jquery.post("http://127.0.0.1:3036/api/person", person ,function (data) {
            console.log(`response from POST request.  data: ${JSON.stringify(data, null, 4)}`)
            jquery("#persons").html(JSON.stringify(data, null, 4));
        });

    });
});


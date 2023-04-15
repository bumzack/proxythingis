import './style.css'
import { setupCounter } from './counter.ts'

import $ from 'jquery';


$( document ).ready(function() {

    const url = "http://www.proxythingi.at/proxythingi/server"
        $.ajax( { url : url, async : false, success: function (output)
        {
            // $(selector).html (output)
            console.log(`servers ${JSON.stringify(output, null, 4)}`);
        }});

    console.log( "yeahhhh!" );
});

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>

    <h1>Vite + TypeScript</h1>
    <div class="card">
      <button id="counter" type="button"></button>
    </div>
    <p class="read-the-docs">
      Click on the Vite and TypeScript logos to learn more
    </p>
  </div>
`

setupCounter(document.querySelector<HTMLButtonElement>('#counter')!)


// $("selector").click (function (){
//     $.ajax( { url : "link", async : false, success: function (output)
//         {
//             $(selector).html (output)
//         }});
// });

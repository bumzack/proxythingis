import './style.css'

import $ from 'jquery';


$(document).ready(function () {

    const url = "http://www.proxythingi.at/proxythingi/server"
    $.ajax({
        url: url, async: false, success: function (output) {
            // $(selector).html (output)
            console.log(`servers ${JSON.stringify(output, null, 4)}`);
        }
    });

    $("#app").html(loadContent());
     console.log("yeahhhh!");
});

const loadContent = ( ): string => {

   return  `

    <header class="navbar navbar-dark sticky-top bg-dark flex-md-nowrap p-0 shadow">
        <a class="navbar-brand col-md-3 col-lg-2 me-0 px-3 fs-6" href="#">Proxy Thingis</a>
        <button class="navbar-toggler position-absolute d-md-none collapsed" type="button" data-bs-toggle="collapse"
                data-bs-target="#sidebarMenu" aria-controls="sidebarMenu" aria-expanded="false"
                aria-label="Toggle navigation">
            <span class="navbar-toggler-icon"></span>
        </button>
        <input class="form-control form-control-dark w-100 rounded-0 border-0" type="text" placeholder="Search"
               aria-label="Search">
        <div class="navbar-nav">
            <div class="nav-item text-nowrap">
                <a class="nav-link px-3" href="#">Sign out</a>
            </div>
        </div>
    </header>
      <div class="container-fluid">
        <div class="row">
            <nav id="sidebarMenu" class="col-md-3 col-lg-2 d-md-block bg-body-tertiary sidebar collapse">
                <div class="position-sticky pt-3 sidebar-sticky">
                    <ul class="nav flex-column">
                        <li class="nav-item">
                            <a class="nav-link" href="#item-1">Item 1</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link ms-3 my-1" href="#item-1-1">Item 1-1</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link ms-3 my-1" href="#item-1-2">Item 1-2</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#item-2">Item 2</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#item-3">Item 3</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">
                                <span data-feather="layers" class="align-text-bottom"></span>
                                Integrations
                            </a>
                        </li>
                    </ul>
    
                    <h6 class="sidebar-heading d-flex justify-content-between align-items-center px-3 mt-4 mb-1 text-body-secondary text-uppercase">
                        <span>Saved reports</span>
                        <a class="link-secondary" href="#" aria-label="Add a new report">
                            <span data-feather="plus-circle" class="align-text-bottom"></span>
                        </a>
                    </h6>
                    <ul class="nav flex-column mb-2">
                        <li class="nav-item">
                            <a class="nav-link" href="#">
                                <span data-feather="file-text" class="align-text-bottom"></span>
                                Current month
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">
                                <span data-feather="file-text" class="align-text-bottom"></span>
                                Last quarter
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">
                                <span data-feather="file-text" class="align-text-bottom"></span>
                                Social engagement
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">
                                <span data-feather="file-text" class="align-text-bottom"></span>
                                Year-end sale
                            </a>
                        </li>
                    </ul>
                </div>
            </nav>
    
            <main class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
                <div class="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom">
                    <h1 class="h2">Proxy Thingis - Server</h1>
                    <div class="btn-toolbar mb-2 mb-md-0">
                        <div class="btn-group me-2">
                            <button type="button" class="btn btn-sm btn-outline-secondary">Share</button>
                            <button type="button" class="btn btn-sm btn-outline-secondary">Export</button>
                        </div>
                        <button type="button" class="btn btn-sm btn-outline-secondary dropdown-toggle">
                            <span data-feather="calendar" class="align-text-bottom"></span>
                            This week
                        </button>
                    </div>
                </div>
    
                <div>
                    <div data-bs-spy="scroll" data-bs-target="#navbar-example3" data-bs-smooth-scroll="true"
                         class="scrollspy-example-2" tabindex="0">
                        <div id="item-1">
                            <h4>Item 1</h4>
                            <p>ore ipsum Lore ipsumLore ipsum Lore ipsum Lore ipsum Lore ipsum Lore ipsum v Lore ipsum Lore
                                ipsum Lore ipsum Lore ipsore ipsum Lore ipsumLore ipsum Lore ipsum Lore ipsum Lore ipsum
                                Lore ipsum v Lore ipsum Lore ipsum Lore ipsum Lore ipsore ipsum Lore ipsumLore ipsum Lore
                                ipsum Lore ipsum Lore ipsum Lore ipsum v Lore ipsum Lore ipsum Lore ipsum Lore ipsore ipsum
                                Lore ipsumLore ipsum Lore ipsum Lore ipsum Lore ipsum Lore ipsum v Lore ipsum Lore ipsum
                                Lore ipsum Lore ipsLore ipsum Lore ipsumLore ipsum Lore ipsum Lore ipsum Lore ipsum Lore
                                ipsum v Lore ipsum Lore ipsum Lore ipsum Lore ipsum v Lore ipsum v Lore ipsum Lore ipsum
                                Lore ipsum Lore ipsum v Lore ipsum v Lore ipsum v sdf </p>
                        </div>
                        <div id="item-1-1">
                            <h5>Item 1-1</h5>
                            <div class="table-responsive">
                                <table class="table table-striped table-sm">
                                    <thead>
                                    <tr>
                                        <th scope="col">#</th>
                                        <th scope="col">Header</th>
                                        <th scope="col">Header</th>
                                        <th scope="col">Header</th>
                                        <th scope="col">Header</th>
                                    </tr>
                                    </thead>
                                    <tbody>
                                    <tr>
                                        <td>1,001</td>
                                        <td>random</td>
                                        <td>data</td>
                                        <td>placeholder</td>
                                        <td>text</td>
                                    </tr>
                                    <tr>
                                        <td>1,002</td>
                                        <td>placeholder</td>
                                        <td>irrelevant</td>
                                        <td>visual</td>
                                        <td>layout</td>
                                    </tr>
                                    <tr>
                                        <td>1,003</td>
                                        <td>data</td>
                                        <td>rich</td>
                                        <td>dashboard</td>
                                        <td>tabular</td>
                                    </tr>
                                    <tr>
                                        <td>1,003</td>
                                        <td>information</td>
                                        <td>placeholder</td>
                                        <td>illustrative</td>
                                        <td>data</td>
                                    </tr>
                                    <tr>
                                        <td>1,004</td>
                                        <td>text</td>
                                        <td>random</td>
                                        <td>layout</td>
                                        <td>dashboard</td>
                                    </tr>
                                    <tr>
                                        <td>1,005</td>
                                        <td>dashboard</td>
                                        <td>irrelevant</td>
                                        <td>text</td>
                                        <td>placeholder</td>
                                    </tr>
                                    <tr>
                                        <td>1,006</td>
                                        <td>dashboard</td>
                                        <td>illustrative</td>
                                        <td>rich</td>
                                        <td>data</td>
                                    </tr>
                                    <tr>
                                        <td>1,007</td>
                                        <td>placeholder</td>
                                        <td>tabular</td>
                                        <td>information</td>
                                        <td>irrelevant</td>
                                    </tr>
                                    <tr>
                                        <td>1,008</td>
                                        <td>random</td>
                                        <td>data</td>
                                        <td>placeholder</td>
                                        <td>text</td>
                                    </tr>
                                    <tr>
                                        <td>1,009</td>
                                        <td>placeholder</td>
                                        <td>irrelevant</td>
                                        <td>visual</td>
                                        <td>layout</td>
                                    </tr>
                                    <tr>
                                        <td>1,010</td>
                                        <td>data</td>
                                        <td>rich</td>
                                        <td>dashboard</td>
                                        <td>tabular</td>
                                    </tr>
                                    <tr>
                                        <td>1,011</td>
                                        <td>information</td>
                                        <td>placeholder</td>
                                        <td>illustrative</td>
                                        <td>data</td>
                                    </tr>
                                    <tr>
                                        <td>1,012</td>
                                        <td>text</td>
                                        <td>placeholder</td>
                                        <td>layout</td>
                                        <td>dashboard</td>
                                    </tr>
                                    <tr>
                                        <td>1,013</td>
                                        <td>dashboard</td>
                                        <td>irrelevant</td>
                                        <td>text</td>
                                        <td>visual</td>
                                    </tr>
                                    <tr>
                                        <td>1,014</td>
                                        <td>dashboard</td>
                                        <td>illustrative</td>
                                        <td>rich</td>
                                        <td>data</td>
                                    </tr>
                                    <tr>
                                        <td>1,015</td>
                                        <td>random</td>
                                        <td>tabular</td>
                                        <td>information</td>
                                        <td>text</td>
                                    </tr>
                                    </tbody>
                                </table>
                            </div>
                            <div id="item-1-2">
                                <h5>Item 1-2</h5>
                                <p>ore ipsum Lore ipsumLore ipsum Lore ipsum Lore ipsum Lore ipsum Lore ipsum v Lore ipsum
                                    Lore ipsum Lore ipsum Lore ipsore ipsum Lore ipsumLore ipsum Lore ipsum Lore ipsum Lore
                                    ipsum Lore ipsum v Lore ipsum Lore ipsum Lore ipsum Lore ipsore ipsum Lore ipsumLore
                                    ipsum Lore ipsum Lore ipsum Lore ipsum Lore ipsum v Lore ipsum Lore ipsum Lore ipsum
                                    Lore ipsore ipsum Lore ipsumLore ipsum Lore ipsum Lore ipsum Lore ipsum Lore ipsum v
                                    Lore ipsum Lore ipsum Lore ipsum Lore ipsLore ipsum Lore ipsumLore ipsum Lore ipsum Lore
                                    ipsum Lore ipsum Lore ipsum v Lore ipsum Lore ipsum Lore ipsum Lore ipsum v Lore ipsum v
                                    Lore ipsum Lore ipsum Lore ipsum Lore ipsum v Lore ipsum v Lore ipsum v sdf </p>
                            </div>
                            <div id="item-2">
                                <h4>Item 2</h4>
                                <p>ore ipsum Lore ipsumLore ipsum Lore ipsum Lore ipsum Lore ipsum Lore ipsum v Lore ipsum
                                    Lore ipsum Lore ipsum Lore ipsore ipsum Lore ipsumLore ipsum Lore ipsum Lore ipsum Lore
                                    ipsum Lore ipsum v Lore ipsum Lore ipsum Lore ipsum Lore ipsore ipsum Lore ipsumLore
                                    ipsum Lore ipsum Lore ipsum Lore ipsum Lore ipsum v Lore ipsum Lore ipsum Lore ipsum
                                    Lore ipsore ipsum Lore ipsumLore ipsum Lore ipsum Lore ipsum Lore ipsum Lore ipsum v
                                    Lore ipsum Lore ipsum Lore ipsum Lore ipsLore ipsum Lore ipsumLore ipsum Lore ipsum Lore
                                    ipsum Lore ipsum Lore ipsum v Lore ipsum Lore ipsum Lore ipsum Lore ipsum v Lore ipsum v
                                    Lore ipsum Lore ipsum Lore ipsum Lore ipsum v Lore ipsum v Lore ipsum v sdf </p>
    
                            </div>
                            <div id="item-3">
                                <h4>Item 3</h4>
                                <p>ore ipsum Lore ipsumLore ipsum Lore ipsum Lore ipsum Lore ipsum Lore ipsum v Lore ipsum
                                    Lore ipsum Lore ipsum Lore ipsore ipsum Lore ipsumLore ipsum Lore ipsum Lore ipsum Lore
                                    ipsum Lore ipsum v Lore ipsum Lore ipsum Lore ipsum Lore ipsore ipsum Lore ipsumLore
                                    ipsum Lore ipsum Lore ipsum Lore ipsum Lore ipsum v Lore ipsum Lore ipsum Lore ipsum
                                    Lore ipsore ipsum Lore ipsumLore ipsum Lore ipsum Lore ipsum Lore ipsum Lore ipsum v
                                    Lore ipsum Lore ipsum Lore ipsum Lore ipsLore ipsum Lore ipsumLore ipsum Lore ipsum Lore
                                    ipsum Lore ipsum Lore ipsum v Lore ipsum Lore ipsum Lore ipsum Lore ipsum v Lore ipsum v
                                    Lore ipsum Lore ipsum Lore ipsum Lore ipsum v Lore ipsum v Lore ipsum v sdf </p>
    
                            </div>
                            <div id="item-3-1">
                                <h5>Item 3-1</h5>
                                <p>...</p>
                            </div>
                            <div id="item-3-2">
                                <h5>Item 3-2</h5>
                                <p>...</p>
                            </div>
                        </div>
                    </div>
    
                    <h2>Section title</h2>
                    <div class="table-responsive">
                        <table class="table table-striped table-sm">
                            <thead>
                            <tr>
                                <th scope="col">#</th>
                                <th scope="col">Header</th>
                                <th scope="col">Header</th>
                                <th scope="col">Header</th>
                                <th scope="col">Header</th>
                            </tr>
                            </thead>
                            <tbody>
                            <tr>
                                <td>1,001</td>
                                <td>random</td>
                                <td>data</td>
                                <td>placeholder</td>
                                <td>text</td>
                            </tr>
                            <tr>
                                <td>1,002</td>
                                <td>placeholder</td>
                                <td>irrelevant</td>
                                <td>visual</td>
                                <td>layout</td>
                            </tr>
                            <tr>
                                <td>1,003</td>
                                <td>data</td>
                                <td>rich</td>
                                <td>dashboard</td>
                                <td>tabular</td>
                            </tr>
                            <tr>
                                <td>1,003</td>
                                <td>information</td>
                                <td>placeholder</td>
                                <td>illustrative</td>
                                <td>data</td>
                            </tr>
                            <tr>
                                <td>1,004</td>
                                <td>text</td>
                                <td>random</td>
                                <td>layout</td>
                                <td>dashboard</td>
                            </tr>
                            <tr>
                                <td>1,005</td>
                                <td>dashboard</td>
                                <td>irrelevant</td>
                                <td>text</td>
                                <td>placeholder</td>
                            </tr>
                            <tr>
                                <td>1,006</td>
                                <td>dashboard</td>
                                <td>illustrative</td>
                                <td>rich</td>
                                <td>data</td>
                            </tr>
                            <tr>
                                <td>1,007</td>
                                <td>placeholder</td>
                                <td>tabular</td>
                                <td>information</td>
                                <td>irrelevant</td>
                            </tr>
                            <tr>
                                <td>1,008</td>
                                <td>random</td>
                                <td>data</td>
                                <td>placeholder</td>
                                <td>text</td>
                            </tr>
                            <tr>
                                <td>1,009</td>
                                <td>placeholder</td>
                                <td>irrelevant</td>
                                <td>visual</td>
                                <td>layout</td>
                            </tr>
                            <tr>
                                <td>1,010</td>
                                <td>data</td>
                                <td>rich</td>
                                <td>dashboard</td>
                                <td>tabular</td>
                            </tr>
                            <tr>
                                <td>1,011</td>
                                <td>information</td>
                                <td>placeholder</td>
                                <td>illustrative</td>
                                <td>data</td>
                            </tr>
                            <tr>
                                <td>1,012</td>
                                <td>text</td>
                                <td>placeholder</td>
                                <td>layout</td>
                                <td>dashboard</td>
                            </tr>
                            <tr>
                                <td>1,013</td>
                                <td>dashboard</td>
                                <td>irrelevant</td>
                                <td>text</td>
                                <td>visual</td>
                            </tr>
                            <tr>
                                <td>1,014</td>
                                <td>dashboard</td>
                                <td>illustrative</td>
                                <td>rich</td>
                                <td>data</td>
                            </tr>
                            <tr>
                                <td>1,015</td>
                                <td>random</td>
                                <td>tabular</td>
                                <td>information</td>
                                <td>text</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </main>
        </div>
    </div>
    `;
}


// $("selector").click (function (){
//     $.ajax( { url : "link", async : false, success: function (output)
//         {
//             $(selector).html (output)
//         }});
// });

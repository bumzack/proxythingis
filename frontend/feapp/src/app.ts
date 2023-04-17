import jquery from "jquery";

declare global {
    interface Window { // ⚠️ notice that "Window" is capitalized here
        $: any;
        jQuery: any;
    }
}

window.$ = window.jQuery = jquery;

jquery(document).ready(() => {
    console.log("yo");
    load_servers();
})

const load_servers = (): void => {
    const url = "http://proxy.proxythingi.at/proxythingi/server";
    jquery.ajax({url: url}).done(data => {
        const sources = data as Array<ServerSource>;
        //  console.log(`sources ${JSON.stringify(sources, null, 4)} `);
        render_server(sources);
    });
}

const source_link_template = (source_id, href_id, txt) => {
    return `
        <li class="nav-item">
            <a class="nav-link" href="#${href_id}" id="${source_id}">${txt}</a>
        </li>
    `;
}


const source_targets_link_list_template = (id) => {
    return `
       <ul class="nav flex-column" id="${id}">
        </ul>
    `;
}

const source_server_content_template = (id, name, source): string => {
    return `
        <div id="${id}" class="card">
          <h4>
            ${name}
          </h4>
            <div>
                ${source_table(source)}
            </div>
            <div>
                ${source_table_stats(source.stats)}
            </div>
        </div>
    `;
}

const target_server_content_template = (id: string, name: string, target: ServerTarget): string => {
    return `
        <div id="${id}" class="card">
          <div class="card-header">
            ${name}
          </div>
          <div class="card-body">
                ${target_table(target)}
          </div>
           <div class="card-body">
                ${target_table_stats(target.stats)}
          </div>
          
        </div>
    `;
}


const source_table = (source: ServerSource): string => {
    return `
        <div class="table-responsive">
            <table class="table table-striped table-sm">
                <thead>
                    <tr>
                        <th scope="col">id</th>
                        <th scope="col">path</th>
                        <th scope="col">method</th>
                        <th scope="col">created</th>
                     </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>${source.id}</td>
                        <td>${source.path_starts_with}</td>
                        <td>${source.method}</td>
                        <td>${source.created}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    `;
}

const source_table_stats = (stats: ServerSourceStats): string => {
    return `
        <div class="table-responsive">
            <table class="table table-striped table-sm">
                <thead>
                    <tr>
                        <th scope="col">id</th>
                        <th scope="col">hits</th>
                        <th scope="col">start</th>
                        <th scope="col">stop</th>
                        <th scope="col">created</th>
                     </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>${stats.id}</td>
                        <td>${stats.hits}</td>
                        <td>${stats.start}</td>
                        <td>${stats.stop}</td>
                        <td>${stats.created}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    `;
}

const target_table = (target: ServerTarget): string => {
    return `
        <div class="table-responsive">
            <table class="table table-striped table-sm">
                <thead>
                    <tr>
                        <th scope="col">id</th>
                        <th scope="col">host</th>
                        <th scope="col">port</th>
                        <th scope="col">path</th>
                        <th scope="col">method</th>
                        <th scope="col">created</th>
                     </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>${target.id}</td>
                        <td>${target.host}</td>
                        <td>${target.port}</td>
                        <td>${target.path}</td>
                        <td>${target.method}</td>
                        <td>${target.created}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    `;
}


const server_target_stat_row = (stat: ServerTargetStats): string => {
    return `
        <tr>
            <td>${stat.id}</td>
            <td>${stat.hits}</td>
            <td>${stat.avg_ns}</td>
            <td>${stat.min_ns}</td>
            <td>${stat.max_ns}</td>
            <td>${stat.created}</td>
            <td>${stat.start}</td>
            <td>${stat.stop}</td>
        </tr>
    `;
}


const target_stats = (stat: ServerTargetStats): string => {
    return `
       <div class="table-responsive">
            <table class="table table-striped table-sm">
                <thead>
                    <tr>
                        <th scope="col">id</th>
                        <th scope="col">hits</th>
                        <th scope="col">avg_ns</th>
                        <th scope="col">min_ns</th>
                        <th scope="col">max_ns</th>
                        <th scope="col">created</th>
                        <th scope="col">start</th>
                        <th scope="col">stop</th>
                     </tr>
                </thead>
                <tbody>
                    ${server_target_stat_row(stat)}
                </tbody>
            </table>
        </div>
    `;
}

const target_table_stats = (t: ServerTargetStats): string => {
    if (t !== null) {
        return target_stats(t);
    }
    return "";
}

const render_server = (sources: Array<ServerSource>) => {
    console.log("render_server");
    jquery("#serversources").empty();
    jquery("#targetservers").empty();
    sources.sort((a, b) => a.id - b.id)
        .forEach(source => {
            // left nav column
            const href_id = `serversources-${source.id}`;
            const txt = `${source.description} // id: ${source.id}`;
            let source_id = `source-${source.id}`;
            const elem = source_link_template(source_id, href_id, txt);

            // console.log(`source.id   ${source.id}`);
            jquery("#serversources").append(elem)
            const id_targets = `serversources-targets-${source.id}`;

            const ul_container_target_links_id = `targets-${source.id}`;
            let ul_container_target_links = source_targets_link_list_template(ul_container_target_links_id)
            // console.log(`jquery("#"+id)   id ${id}    ${jquery("#" + id)}`);
            jquery("#" + source_id).append(ul_container_target_links);
            // console.log(`id_targets ${id_targets}  s ${s}`);


            // content column
            let content_source_id = `content-source-${source.id}`;
            let content = source_server_content_template(content_source_id, txt, source);
            // console.log(`content ${content}   `);
            jquery("#targetservers").append(content);

            source.targets.sort((a, b) => a.id - b.id)
                .forEach(target => {
                    const target_id = `serversources-${source.id}-${target.id}`;

                    const link_origin = source_link_template("id-for-" + target_id, target_id, target.description);
                    // console.log(`link_origin ${link_origin}  target_id ${target_id} `);
                    const t = "#" + ul_container_target_links_id;
                    jquery(t).append(link_origin);


                    // content column
                    const target_txt = `${target.description} // id: ${target.id}`;
                    const target_elem = target_server_content_template(target_id, target_txt, target);
                    // console.log(`target_elem ${target_elem}  target_id ${target_id} `);

                    const id = "#" + content_source_id;
                    console.log(`id ${id}     target_elem ${target_elem}`);
                    jquery(id).append(target_elem);
                })
        });
}


export interface ServerSourceStats {
    id: number,
    source_id: number,
    hits: number,
    start: string,
    stop: string,
    created: string,
}

export interface ServerTargetStats {
    id: number,
    target_id: number,
    hits: number,
    avg_ns: number,
    max_ns: number,
    min_ns: number,
    start: string,
    stop: string,
    created: string,
}


export interface ServerSource {
    id: number,
    description: string,
    path_starts_with: string
    method: string,
    created: string,
    targets: Array<ServerTarget>
    stats: ServerSourceStats,
}

export interface ServerTarget {
    id: number
    description: string,
    schema: string,
    host: string,
    port: number,
    path: string,
    method: string,
    stats: ServerTargetStats,
    active: boolean,
    created: string,
}


export {};
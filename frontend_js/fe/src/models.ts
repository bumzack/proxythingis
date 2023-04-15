export interface server_source_stats {
    id: number,
    source_id: number,
    hits: number,
    start: string,
    stop: string,
    created: string,
}

export interface server_target_stats {
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


export interface server_source {
    id: number,
    description: string,
    path_starts_with: string
    method: string,
    created: string,
    targets: Array<server_target>
    stats: server_source_stats,
}

export interface server_target {
    id: number
    description: string,
    schema: string,
    host: string,
    port: number,
    path: string,
    method: string,
    stats:Array<server_target_stats>,
    active: boolean,
    created: string,
}

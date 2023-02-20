use core::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;
use super::data::{ProcessList, SearchOptions, ProcessStreams, empty_process_list};
use super::handlers;

pub fn processbrowser() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let stored = empty_process_list();
    let streams = Arc::new(Mutex::new(Vec::new()));
    get_processes(stored.clone())
        .or(search_processes(stored.clone()))
        .or(acquire_processes(stored, streams.clone()))
        .or(stream_processes(streams))
}

fn get_processes(plist: ProcessList) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("processes")
        .and(warp::get())
        .and(with_process_list(plist))
        .and_then(handlers::list_processes)
}

fn search_processes(plist: ProcessList) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("search")
        .and(warp::get())
        .and(warp::query::<SearchOptions>())
        .and(with_process_list(plist))
        .and_then(handlers::search_processes)
}

fn acquire_processes(plist: ProcessList, streams: ProcessStreams) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("acquire_process_list")
        .and(warp::post())
        .and(with_process_list(plist))
        .and(with_process_streams(streams))
        .and_then(handlers::acquire_processes)
}

fn stream_processes(streams: ProcessStreams) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("data")
        .and(warp::get())
        .and(with_process_streams(streams))
        .and_then(handlers::stream_processes)
}

fn with_process_list(plist: ProcessList) -> impl Filter<Extract = (ProcessList,), Error = Infallible> + Clone {
    warp::any().map(move || plist.clone())
}

fn with_process_streams(streams: ProcessStreams) -> impl Filter<Extract = (ProcessStreams,), Error = Infallible> + Clone {
    warp::any().map(move || streams.clone())
}

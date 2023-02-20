use core::convert::Infallible;
use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::sse::Event;
use super::data::{ProcessList, ProcessData, SearchOptions, ProcessStreams};
use super::sys;

pub async fn list_processes(plist: ProcessList) -> Result<(impl warp::Reply,), Infallible> {
    let processes = plist.lock().await.clone();
    Ok((warp::reply::json(&processes),))
}

fn check_valid(process: &ProcessData, search: &SearchOptions) -> bool {
    (search.pid.is_none() || Some(process.pid) == search.pid) && (search.username.is_none() || process.username == search.username)
}

pub async fn search_processes(search: SearchOptions, plist: ProcessList) -> Result<(impl warp::Reply,), Infallible> {
    let processes: Vec<ProcessData> = plist.lock().await.clone()
        .into_iter()
        .filter(|process| check_valid(process, &search))
        .collect();
    Ok((warp::reply::json(&processes),))
}

pub async fn publish(streams: ProcessStreams, process: ProcessData) {
    tokio::spawn(async move {
        streams.lock().await.retain(|stream| stream.send(process.clone()).is_ok())
    });
}

pub async fn acquire_processes(plist: ProcessList, streams: ProcessStreams) -> Result<(impl warp::Reply,), Infallible> {
    let mut processes = plist.lock().await;
    let current_pids: Vec<u32> = processes.iter().map(|process| process.pid).collect();
    *processes = sys::current_processes();
    let new_processes: Vec<&ProcessData> = processes.iter()
        .filter(|process| !current_pids.contains(&process.pid))
        .collect();
    for process in new_processes {
        publish(streams.clone(), process.clone()).await;
    }
    Ok((warp::http::StatusCode::CREATED,))
}

pub async fn stream_processes(streams: ProcessStreams) -> Result<(impl warp::Reply,), Infallible> {
    let stream = join_stream(streams).await;
    Ok((warp::sse::reply(warp::sse::keep_alive().stream(stream)),))
}

async fn join_stream(streams: ProcessStreams) -> impl Stream<Item = Result<warp::sse::Event, warp::Error>> + Send + 'static {
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    streams.lock().await.push(tx);

    rx.map(|process| Ok(Event::default().event("process").json_data(process).unwrap()))
}

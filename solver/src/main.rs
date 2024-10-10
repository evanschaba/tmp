use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    thread,
};
use tokio::runtime::Runtime;
use async_graphql::{Schema, Object, Context};
use async_graphql_warp::{GraphQLResponse, GraphQLRequest};
use warp::{Filter, http::Response};
use futures::future::join_all;
use serde::{Serialize, Deserialize};
use rmp_serde::{encode, decode};
use std::sync::mpsc::{channel, Sender, Receiver};
use quinn::{Endpoint, ServerConfig};

// State shared between the main thread and workers
#[derive(Debug, Clone)]
struct RequestState {
    thread_id: usize,
    result: Option<String>,
}

type RequestMap = Arc<Mutex<HashMap<u64, RequestState>>>;

#[derive(Default)]
struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn process_request(&self, ctx: &Context<'_>, req_id: u64) -> String {
        let request_map = ctx.data::<RequestMap>().unwrap();
        let request_map_lock = request_map.lock().unwrap();

        if let Some(state) = request_map_lock.get(&req_id) {
            if let Some(result) = &state.result {
                return result.clone();
            } else {
                return format!("Request {} is being processed on thread {}", req_id, state.thread_id);
            }
        } else {
            return format!("Request ID {} not found.", req_id);
        }
    }
}

#[tokio::main]
async fn main() {
    let request_map: RequestMap = Arc::new(Mutex::new(HashMap::new()));

    let schema = Schema::build(QueryRoot::default(), async_graphql::EmptyMutation, async_graphql::EmptySubscription)
        .data(request_map.clone())
        .finish();

    // Start multiple HTTP/3 servers in separate threads
    let (tx, rx): (Sender<(u64, usize, String)>, Receiver<(u64, usize, String)>) = channel();

    let mut handles = vec![];
    for i in 1..=4 {
        let tx = tx.clone();
        let request_map = request_map.clone();
        let handle = thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                start_server(tx, request_map, i, 8080 + i).await;
            });
        });
        handles.push(handle);
    }

    // Handle responses from worker threads
    let request_map_clone = request_map.clone();
    let main_handle = thread::spawn(move || {
        loop {
            if let Ok((req_id, thread_id, result)) = rx.recv() {
                let mut request_map = request_map_clone.lock().unwrap();
                if let Some(state) = request_map.get_mut(&req_id) {
                    state.result = Some(result);
                }
            }
        }
    });

    // GraphQL endpoint
    let graphql_filter = warp::path("graphql")
        .and(async_graphql_warp::graphql(schema))
        .and_then(|(schema, request): (Schema<QueryRoot, _, _>, GraphQLRequest)| async move {
            let resp = schema.execute(request.into_inner()).await;
            Ok::<_, warp::Rejection>(GraphQLResponse::from(resp))
        });

    // GraphiQL interface
    let graphiql_filter = warp::path("graphiql").map(|| {
        warp::reply::html(async_graphql_warp::graphiql_source("/graphql"))
    });

    // Warp server
    warp::serve(graphql_filter.or(graphiql_filter))
        .run(([0, 0, 0, 0], 8000))
        .await;

    for handle in handles {
        handle.join().unwrap();
    }
    main_handle.join().unwrap();
}

// Start individual HTTP/3 servers
async fn start_server(tx: Sender<(u64, usize, String)>, request_map: RequestMap, thread_id: usize, port: u16) {
    let addr = format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap();

    let mut config = ServerConfig::default();
    config.certificate = quinn::CertificateChain::from_pem("server_cert.pem").unwrap();
    config.private_key = quinn::PrivateKey::from_pem("server_key.pem").unwrap();

    let mut endpoint = Endpoint::builder();
    endpoint.listen(config);

    let (endpoint, mut incoming) = endpoint.bind(&addr).expect("Failed to bind server");

    println!("Server listening on {}", addr);

    while let Some(connecting) = incoming.next().await {
        match handle_connection(connecting).await {
            Ok((req_id, result)) => {
                // Send response back to main thread
                if tx.send((req_id, thread_id, result)).is_err() {
                    eprintln!("Main thread has stopped receiving responses.");
                }
            }
            Err(e) => eprintln!("Error handling connection: {}", e),
        }
    }
}

async fn handle_connection(connecting: quinn::Connecting) -> Result<(u64, String), quinn::ConnectionError> {
    let connection = connecting.await?;
    let addr = connection.remote_address();

    // Simulate a request ID
    let req_id = rand::random::<u64>();

    // Simulate processing a request
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    Ok((req_id, format!("Response from {}", addr)))
}

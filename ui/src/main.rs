use std::error;
use gloo_net::http::Request;
use gloo_net::websocket::{Message, futures::WebSocket};
use futures::StreamExt;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[function_component(SlotNum)]
fn slot_num() -> Html {
    let slot = use_state(|| None);

    {
        let slot = slot.clone();
        use_effect(move || {
            if slot.is_none() {
                let ws = WebSocket::open("ws://localhost:8000/slot").unwrap();
                let (_, mut read) = ws.split();

                spawn_local(async move {
                    while let Some(msg) = read.next().await {
                        let result = msg.unwrap();
                        match result {
                            Message::Text(text) => {
                                slot.set(Some(text));
                            },
                            _ => {},
                        };
                    }
                });
            }
            || {}
        });
    }

    html! {
        <div id="slot">
            { 
              match slot.as_ref() {
                None => html! {},
                Some(data) => html! { format!("Slot: {}", data) },
              }
            }
        </div>
    }
}

#[function_component(EpochNum)]
fn epoch_num() -> Html {
    let epoch = use_state(|| None);

    {
        let epoch = epoch.clone();
        use_effect(move || {
            if epoch.is_none() {
                spawn_local(async move {
                    let result = get_data("http://localhost:8000/epoch").await;
                    epoch.set(Some(result));
                });
            }
            || {}
        });
    }

    match epoch.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <div>{"Epoch: "}{data}</div>
            }
        }
        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from server: "}{err}</div>
            }
        }
    }
}

async fn get_data(url: &str) -> Result<String> {
	let resp = Request::get(url).send().await?;
	let result = {
		if !resp.ok() {
			Err(format!(
				"Error fetching data {} ({})",
				resp.status(),
				resp.status_text()
			))
		} else {
			resp.text().await.map_err(|err| err.to_string())
		}
	};

	Ok(result?)
}


#[function_component(App)]
fn app() -> Html {
	html! { 
		<>
			<SlotNum /> 
			<EpochNum />
		</>
	}
}


fn main() {
    yew::Renderer::<App>::new().render();
}

use std::error;
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
                let ws = WebSocket::open("ws://localhost:8000/epoch").unwrap();
                let (_, mut read) = ws.split();

                spawn_local(async move {
                    while let Some(msg) = read.next().await {
                        let result = msg.unwrap();
                        match result {
                            Message::Text(text) => {
                                epoch.set(Some(text));
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
        <div id="epoch">
            { 
              match epoch.as_ref() {
                None => html! {},
                Some(data) => html! { format!("Epoch: {}", data) },
              }
            }
        </div>
    }
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

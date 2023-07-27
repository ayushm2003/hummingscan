use std::error;
use gloo_net::websocket::{Message, futures::WebSocket};
use futures::StreamExt;
// use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
// use yew_hooks::prelude::*;
// use web_sys::console;

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

#[function_component(ValidatorNum)]
fn validator_num() -> Html {
	let val_num = use_state(|| None);
	let act_val_num = use_state(|| None);

	{
        let val_num = val_num.clone();
		let act_val_num = act_val_num.clone();

        use_effect(move || {
            if val_num.is_none() || act_val_num.is_none() {
                let ws = WebSocket::open("ws://localhost:8000/validator").unwrap();
                let (_, mut read) = ws.split();

                spawn_local(async move {
                    while let Some(msg) = read.next().await {
                        let result = msg.unwrap();
                        match result {
                            Message::Text(text) => {
								// let (num, act_num) = text.parse::<(usize, usize)>().unwrap();
								let mut parts = text.split(',');
								let num_str = parts.next().unwrap().trim().trim_start_matches('(');
								let act_num_str = parts.next().unwrap().trim().trim_end_matches(')');
								//console::log_1(&JsValue::from(num_str));

								let num = num_str.trim().parse::<usize>().unwrap();
								let act_num = act_num_str.trim().parse::<usize>().unwrap();
								
                                val_num.set(Some(num));
								act_val_num.set(Some(act_num));
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
		<>
			<div id="val_num">
				{ 
				match val_num.as_ref() {
					None => html! {},
					Some(data) => html! { format!("Validators: {}", data) },
				}
				}
			</div>
			<div id="act_val_num">
				{
					match act_val_num.as_ref() {
						None => html! {},
						Some(data) => html! { format!("Active validators: {}", data) },
					}
				}
			</div>
		</>
	}
}


#[function_component(App)]
fn app() -> Html {
	html! { 
		<>
			<SlotNum /> 
			<EpochNum />
			<ValidatorNum />
		</>
	}
}


fn main() {
    yew::Renderer::<App>::new().render();
}

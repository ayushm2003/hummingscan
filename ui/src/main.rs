use gloo_net::http::{Request};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(SlotNum)]
fn slot_num() -> Html {
    let slot = use_state(|| None);

    {
        let slot = slot.clone();
        use_effect(move || {
            if slot.is_none() {
                spawn_local(async move {
                    let resp = Request::get("http://localhost:8000/slot").send().await.unwrap();
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
                    slot.set(Some(result));
                });
            }

            || {}
        });
    }

    match slot.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <div>{"Slot: "}{data}</div>
            }
        }
        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from server: "}{err}</div>
            }
        }
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
                    let resp = Request::get("http://localhost:8000/epoch").send().await.unwrap();
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

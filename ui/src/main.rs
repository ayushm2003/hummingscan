use std::error;
use gloo_net::http::{Request};
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
                spawn_local(async move {
                    let result = get_data("http://localhost:8000/slot").await;
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

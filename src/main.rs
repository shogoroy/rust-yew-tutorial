mod components;
mod domain;

use components::entity_components::video_details::VideoDetails;
use components::entity_components::videos_list::VideosList;
use domain::entities::video::model::Video;
use gloo_net::http::Request;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with((), move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                videos.set(fetched_videos);
            });
            || ()
        });
    }
    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    html! {
      <>
        <h1>{ "RustConf Explorer" }</h1>
        <div>
            <h3>{"Videos to watch"}</h3>
            <VideosList videos={(*videos).clone()} on_click={on_video_select} />
        </div>
        {for details}
      </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

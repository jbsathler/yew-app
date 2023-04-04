use yew::prelude::*;
use gloo_net::http::Request;

use crate::video::{
  video_model::Video,
  components::{
    videos_list::VideosList,
    video_details::VideoDetails,
  },
};

mod video;

#[function_component(App)]
fn app() -> Html {
  let videos = use_state(|| vec![]);
  {
    let videos = videos.clone();
    use_effect_with_deps(move |_| {
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
    }, ());
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
      <h1 class={classes!("text-4xl", "mt-8", "font-bold")}>{ "RustConf Explorer" }</h1>
      <div class={classes!("mt-8")}>
        <h3 class={classes!("text-2xl", "font-semibold", "mb-4")}>{ "Videos to watch" }</h3>
        <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()}  />
      </div>
      <div class={classes!("mt-8")}>
        { for details }
      </div>
    </>
  }
}
fn main() {
  yew::Renderer::<App>::new().render();
}

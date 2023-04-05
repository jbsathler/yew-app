use yew::prelude::*;

use crate::video::video_model::Video;

#[derive(Properties, PartialEq)]
pub struct VideoDetailsProps {
  pub video: Video,
}

fn get_embed_link(url: String) -> String {
  return url.clone().replace("https://youtu.be/", "https://www.youtube-nocookie.com/embed/");
}

#[function_component(VideoDetails)]
pub fn video_details(VideoDetailsProps { video }: &VideoDetailsProps) -> Html {
  html! {
    <div>
      <h3 class={classes!("text-2xl", "font-semibold", "mb-2")}>{ video.title.clone() }</h3>
      <iframe width="640" height="360" frameborder="0" allowfullscreen={true} title="YouTube video player"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
        src={ get_embed_link(video.url.clone()) }
      ></iframe>
    </div>
  }
}

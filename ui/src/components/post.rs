use chrono::{DateTime, LocalResult, TimeZone, Utc};
use types::PostResponse;
use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct PostProperties {
    pub content: PostResponse,
    pub now: DateTime<Utc>,
}

fn get_relative_date(delta: i64) -> Option<String> {
    let secs_per_min = 60;
    let secs_per_hour = secs_per_min * 60;
    let secs_per_day = secs_per_hour * 24;
    if delta == 0 {
        Some(String::from("now"))
    } else if delta < secs_per_min {
        Some(format!("{}s", delta))
    } else if delta < secs_per_hour {
        Some(format!("{}m", delta / secs_per_min))
    } else if delta < secs_per_day {
        Some(format!("{}h", delta / secs_per_hour))
    } else {
        None
    }
}

#[function_component(Post)]
pub fn post_component(props: &PostProperties) -> Html {
    let PostProperties { content, now } = props;

    let timestamp = if let LocalResult::Single(dt) = Utc.timestamp_opt(content.date, 0) {
        dt.naive_local().to_string()
    } else {
        content.date.to_string()
    };

    let display_date = if let Some(d) = get_relative_date(now.timestamp() - content.date) {
        html! {
              <div class="relative group">
              <div class="opacity-0 group-hover:opacity-100">
              {timestamp}
              </div>
              <div class="opacity-100 group-hover:opacity-0 absolute top-0">
              {d}
                  </div>
                  </div>
        }
    } else {
        html! {
            <div>
            {timestamp}
            </div>
        }
    };

    html! {
        <div class="relative bg-darkish-purple p-6 max-w-full break-words text-gray-500 border-l-4 border-grey-purple rounded-md mb-4 drop-shadow-xl hover:border-hot-pink hover:scale-[1.01] hover:drop-shadow-xxl transition">
            <div class="font-sans flex flex-row z-10">
      <div class="flex-none w-1/3 max-h-96 z-10">
        <img src={format!("http://{}:{}/{}", std::env!("API_HOST"), std::env!("API_PORT"), content.img_path.to_owned())}
            class="w-full h-full object-center object-scale-down rounded-lg drop-shadow-lg"/>
      </div>
      <div class="flex flex-col shrink w-full max-h-96 pl-4 overflow-y-auto z-10">
          <h2 class="text-lg font-bold">{content.title.to_owned()}</h2>
          <p class="text-white">{display_date}</p>
          <div class="h-full w-full mt-2 rounded-md bg-purple">
        <p class="text-white p-2 md:text-sm text-xs"> {content.text.to_owned()}</p>
        </div>
        </div>
    </div>
    </div>

        }
}

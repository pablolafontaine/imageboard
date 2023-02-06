use types::PostResponse;
use yew::{function_component, html, Html, Properties};
use chrono::{Utc, TimeZone, LocalResult, DateTime};

#[derive(PartialEq, Properties)]
pub struct PostProperties {
    pub content: PostResponse,
    pub now: DateTime<Utc>,
}

fn get_relative_date(delta: i64) -> Option<String>{
    let secs_per_min = 60;
    let secs_per_hour = secs_per_min * 60;
    let secs_per_day = secs_per_hour * 24;
   if delta == 0{
        Some(String::from("now"))
   }
   else if delta < secs_per_min{
        Some(format!("{}s", delta/1000))
   }
   else if delta < secs_per_hour{
        Some(format!("{}m", delta/secs_per_min))
   }
   else if delta < secs_per_day{
        Some(format!("{}h", delta/secs_per_hour))
   }
   else{
        None
   }
}

#[function_component(Post)]
pub fn post_component(props: &PostProperties) -> Html {
    let PostProperties { content, now } = props;

    let timestamp = if let LocalResult::Single(dt) = Utc.timestamp_opt(content.date, 0){
        dt.naive_local().to_string()
    }
    else{
        content.date.to_string()
    };

    let display_date = if let Some(d) = get_relative_date(now.timestamp() - content.date) {
      html! {
            <div class="group">
            {timestamp}
            <div class="scale-100 group-hover:scale-100">
            {d}
                </div>
                </div>

      }  
    }
    else{
        html!{
            <div>
            {timestamp}
            </div>
        }
    };
    
    html! {
        <div class="pt-4 pb-4 mt-4 mb-4 bg-gray-700 rounded-lg">
            <div class="ml-4 mr-4 inline-block">
            <div class="name">{content.title.to_owned()}</div>
            {display_date}
            <div class="max-w-xl max-h-[36rem]">
            <div class="pr-4 mr-4">
            <img class="max-h-full w-1/4 float-left pr-4 rounded-sm" src={format!("http://{}:{}/{}", std::env!("API_HOST"), std::env!("API_PORT"), content.img_path.to_owned())}/>
            <div class="ml-4">{content.text.to_owned()}</div>
            </div>
            </div>
            </div>
        </div>
    }
}


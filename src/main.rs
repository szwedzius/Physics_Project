mod camera;
mod plot;
mod imageDiff;
mod randomNumberComp;

use yew::prelude::*;
use camera::Camera;
use plot::Plot;
use imageDiff::ImageDiff;
use randomNumberComp::RandomNumberComp;


#[function_component]
fn App() -> Html {
    html! {
        <div>
            <Plot />
            <Camera />
            <RandomNumberComp />
            <ImageDiff />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

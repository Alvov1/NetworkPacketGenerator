
use crate::model::Model;
use crate::view::View;

struct Controller {
    view: View,
    model: Model
}
impl Controller {
    fn new(view: View, model: Model) -> Controller {
        Controller { view, model }
    }
}
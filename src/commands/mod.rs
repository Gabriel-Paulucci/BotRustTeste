mod util;

use serenity::framework::StandardFramework;

pub fn get_framework() -> StandardFramework {
    StandardFramework::new()
        .configure(|x| x.prefix("~"))
        .group(&util::UTIL_GROUP)
}
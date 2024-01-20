# visionos-checker

Really hacky script to see which apps on a top chart will be compatible with visionOS

Inspired by [this article on MacStories](https://www.macstories.net/news/a-survey-of-popular-apps-currently-compatible-with-apple-vision-pro/). Note that, as John Voorhees writes:

> Before we get to our findings below, a **word of caution**. An opted-out app may become available in the future. A good example is Disney+, which was [demoed by Apple](https://www.macstories.net/news/new-apple-vision-pro-hands-on-accounts-from-engadget-and-the-verge/) to members of the press earlier this week. Disney’s app currently shows that the company has opted out of Vision Pro compatibility. Instead of indicating that Disney is taking a pass on the device, this is most likely because the app isn’t ready yet, and an app update will flip the switch to make it Vision Pro-compatible.

How to use:
1. [Install Rust](https://www.rust-lang.org/learn/get-started) if you don't have it already
2. `cargo run https://apps.apple.com/us/charts/iphone/food-drink-apps/6023?chart=top-free` or something to that effect
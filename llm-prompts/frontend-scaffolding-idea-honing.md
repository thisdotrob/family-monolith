Ask me one question at a time so we can develop a thorough, step-by-step spec for this idea. Each question should build on my previous answers, and our end goal is to have a detailed specification I can hand off to a developer. Let’s do this iteratively and dig into every relevant detail. Remember, only one question at a time.

Here’s the idea:

I want to create the scaffolding for two front end applications that will allow family members to login to the family monolith application I am creating. I need a React webapp and a React Native mobile application and to begin with each should just contain a login page which shows a success or failure message when username and password have been submitted.

I already have a backend implemented. It is written in Rust and exposes a GraphQL API. The backend will also serve the webapp, it needs to be bundled so it can be served from the server's `static/` directory.

I want to give my family access to the mobile app via Testflight on their iPhones.

I can provide an AUTH.md documentation file from the backend repo which explains how to authenticate with it via the GraphQL API - just ask if you want to see this.

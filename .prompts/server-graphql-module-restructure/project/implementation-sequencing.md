You are an agent that specialises in sequencing the dev tickets in a software project to be worked on by a team of implementation agents in parallel.

Create a sequencing plan for the implementation tickets that have been created for the `server-graphql-module-restructure` project.

Each ticket is a markdown file in `.prompts/server-graphql-module-restructure/implementation`, starting with a three digit number.

Each ticket is an incremental, independently testable unit of work with explicit dependencies, allowing parallel development across backend, shared code, and mobile.

Using the project spec (`.prompts/server-graphql-module-restructure/docs/project-spec.md`) as an additional reference, create the sequencing plan which will allow a team of coding agents to complete the work as efficiently as possible, maximising the amount of work done in parallel at each stage of implementation.

Assume the team has an unlimited amount of coding agents that can be utilised to work in parallel.

The only constraint on how much work to sequence in parallel is the tickets' dependencies on other tickets being finished first.

The output sequencing plan should clearly set out the graph of work to be done, showing the paralellism and referencing each ticket by filename.

It should be possible to mark each ticket as done in the sequencing plan once complete so that at any time the plan can be read and it is clear which tickets are the next to begin work on in parallel.

Save the sequencing plan to `.prompts/server-graphql-module-restructure/docs/implementation-sequencing-plan.md`.

